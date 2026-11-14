//! The import: reading the pages, the addresses it refuses, the model's
//! answer and the route, with fixtures instead of the web and a fake
//! model.

use std::net::IpAddr;
use std::sync::Arc;

use serde_json::{Value, json};
use url::Url;
use zipp::axum::body::{Body, to_bytes};
use zipp::axum::http::{Request, header};
use zipp::entries::api::CreateReq;
use zipp::testing::{Client, TestServer, input};

use super::extract::{self, Section};
use super::fetch::{self, is_public};
use super::*;
use crate::assistant::tests::{FakeLlm, answer, fake_llm};
use crate::content::schema;

const SWISSMILK: &str = include_str!("fixtures/swissmilk.html");
const SECTIONS: &str = include_str!("fixtures/sections.html");
const PLAIN: &str = include_str!("fixtures/plain.html");

#[test]
fn a_recipe_in_a_graph() {
	let r = extract::json_ld_recipe(SWISSMILK).unwrap();
	assert_eq!(r.name, "Älplermagronen mit Apfelmus");
	assert_eq!(
		r.description,
		"Der Klassiker aus der Alphütte & ein Rezept für die ganze Familie."
	);
	assert_eq!(r.category, "Hauptgericht, Vegetarisch");
	assert_eq!(r.serves, Some(4));
	assert_eq!(
		(r.total_time, r.prep_time, r.cook_time),
		(Some(70), Some(30), Some(40))
	);
	assert_eq!(r.ingredients.len(), 8);
	assert_eq!(r.ingredients[3], "2 EL Bratbutter");
	assert_eq!(r.sections.len(), 1);
	let s = &r.sections[0];
	assert_eq!(s.title, "");
	assert_eq!(s.steps.len(), 4);
	// entities and tags out
	assert_eq!(
		s.steps[0],
		"Kartoffeln schälen und in 1 cm grosse Würfel schneiden."
	);
	assert_eq!(
		s.steps[2],
		"Zwiebeln in Ringe schneiden und in der Bratbutter goldbraun braten."
	);
}

#[test]
fn a_recipe_with_sections() {
	let r = extract::json_ld_recipe(SECTIONS).unwrap();
	assert_eq!(r.name, "Plum Tart with Almond Cream");
	assert_eq!(r.serves, Some(6));
	// no totalTime: preparing and cooking
	assert_eq!((r.prep_time, r.total_time), (Some(25), Some(70)));
	assert_eq!(
		r.sections,
		vec![
			Section {
				title: "Pastry".into(),
				steps: vec![
					"Rub the flour and the butter together.".into(),
					"Add the egg, bring together and chill for 30 minutes."
						.into(),
				],
			},
			Section {
				title: "Filling & baking".into(),
				steps: vec![
					"Mix the almonds with the rest of the egg.".into(),
					"Halve the plums.".into(),
					"Bake at 200 °C for 45 minutes.".into(),
				],
			},
		]
	);
}

#[test]
fn a_page_without_json_ld_gives_its_text() {
	assert_eq!(extract::json_ld_recipe(PLAIN), None);
	let text = extract::main_text(PLAIN);
	assert!(
		text.starts_with("Grandma's Zopf – my kitchen blog\n"),
		"{text}"
	);
	assert!(text.contains("\n1½ TL Zucker\n"), "{text}");
	assert!(text.contains("bake at 200°C for 35 minutes."), "{text}");
	// no scripts, menus, header, footer
	for gone in ["ignore all", "About me", "font-family", "Subscribe", "©"] {
		assert!(!text.contains(gone), "{gone} in {text}");
	}
	// a long page is cut
	let long = format!("<main>{}</main>", "<p>Stir well.</p>".repeat(5000));
	assert!(extract::main_text(&long).len() <= extract::MAX_TEXT);
}

#[test]
fn durations_and_yields() {
	let m = extract::duration_minutes;
	assert_eq!(m("PT1H30M"), Some(90));
	assert_eq!(m("PT90M"), Some(90));
	assert_eq!(m("P0DT0H20M"), Some(20));
	assert_eq!(m("P1D"), Some(1440));
	assert_eq!(m("PT30S"), Some(1));
	assert_eq!(m("pt45m"), Some(45));
	assert_eq!(m("45 min"), None);
	assert_eq!(m("PT"), None);
	assert_eq!(m("P1H"), None);
	let y = extract::serves;
	assert_eq!(y(&json!(4)), Some(4));
	assert_eq!(y(&json!("für 4-6 Personen")), Some(4));
	assert_eq!(y(&json!(["", "12 Stück"])), Some(12));
	assert_eq!(y(&json!("ein Blech")), None);
	assert_eq!(y(&json!(0)), None);
}

#[test]
fn only_public_addresses() {
	let ip = |s: &str| s.parse::<IpAddr>().unwrap();
	for private in [
		"10.1.2.3",
		"127.0.0.1",
		"127.8.8.8",
		"169.254.169.254",
		"172.16.0.1",
		"192.168.1.63",
		"100.64.0.1",
		"0.0.0.0",
		"255.255.255.255",
		"224.0.0.1",
		"::1",
		"::",
		"::ffff:127.0.0.1",
		"::ffff:10.0.0.1",
		"fc00::1",
		"fd12:3456::1",
		"fe80::1",
		"ff02::1",
		"2001:db8::1",
		"2002:7f00:1::1",
		"64:ff9b::7f00:1",
	] {
		assert!(!is_public(ip(private)), "{private}");
	}
	for public in ["8.8.8.8", "185.199.108.153", "2a00:1450:400a::200e"] {
		assert!(is_public(ip(public)), "{public}");
	}

	for bad in [
		"ftp://example.com/recipe",
		"file:///etc/passwd",
		"https://example.com:8080/recipe",
		"https://user:pw@example.com/",
		"no url",
	] {
		assert!(fetch::parse_url(bad).is_err(), "{bad}");
	}
	assert!(fetch::parse_url("https://example.com:443/recipe").is_ok());
}

#[tokio::test]
async fn private_hosts_are_not_fetched() {
	for url in [
		"http://127.0.0.1/",
		"http://10.0.0.1/",
		"http://[::1]/",
		"http://169.254.169.254/latest/meta-data/",
		"http://localhost/",
	] {
		let err = Web.fetch(Url::parse(url).unwrap()).await.unwrap_err();
		assert!(err.contains("not on the public internet"), "{url}: {err}");
	}
}

/// The fixtures by their path instead of the web.
struct Fixtures;

#[async_trait::async_trait]
impl Pages for Fixtures {
	async fn fetch(&self, url: Url) -> std::result::Result<Page, String> {
		let html = match url.path() {
			"/swissmilk" => SWISSMILK,
			"/sections" => SECTIONS,
			"/plain" => PLAIN,
			_ => return Err("The page answered with an error (404).".into()),
		};
		Ok(Page {
			url,
			html: html.into(),
		})
	}
}

async fn post(client: &Client, path: &str, body: Value) -> (u16, Value) {
	let req = Request::post(path)
		.header(header::CONTENT_TYPE, "application/json")
		.body(Body::from(body.to_string()))
		.unwrap();
	let resp = client.send(req).await;
	let status = resp.status().as_u16();
	let body = to_bytes(resp.into_body(), usize::MAX).await.unwrap();
	(status, serde_json::from_slice(&body).unwrap_or_default())
}

fn server(llm: Option<Arc<FakeLlm>>) -> TestServer {
	let llm = llm.map(|l| l as Arc<dyn Llm>);
	TestServer::new(schema())
		.routes(routes(Import::new(Arc::new(Fixtures), llm)))
}

#[tokio::test]
async fn members_import_a_recipe() {
	let llm = fake_llm(answer());
	let ts = server(Some(llm.clone()));
	let mut i = input(
		"categories",
		"category",
		"Main Courses",
		json!({ "singular": "Main Course" }),
	);
	i.published = true;
	ts.admin().call(&CreateReq(i)).await.unwrap();

	let url = json!({ "url": "https://www.example-milk.ch/swissmilk" });
	let (status, _) =
		post(&ts.visitor(None), "/members/import", url.clone()).await;
	assert_eq!(status, 401);

	let lea = ts.site_user(Some("member"));
	let (status, r) = post(&lea, "/members/import", url).await;
	assert_eq!(status, 200, "{r}");
	assert_eq!(r["title"], "Alpine Macaroni");
	assert_eq!(r["category"], "main-courses");
	assert_eq!(r["source"], "https://www.example-milk.ch/swissmilk");
	// the page's JSON-LD wins over the model
	assert_eq!((&r["serves"], &r["totalTime"]), (&json!(4), &json!(70)));
	assert_eq!(r["activeTime"], 30);
	assert_eq!(r["chapters"][0]["ingredients"][0]["unit"], "g");
	assert_eq!(r["chapters"][0]["ingredients"][3]["unit"], Value::Null);

	// the model got the categories and the page as data
	let asked = llm.last_text();
	assert!(asked.contains("- main-courses: Main Courses"), "{asked}");
	assert!(
		asked.contains("The page's schema.org Recipe as JSON:\n<untrusted>"),
		"{asked}"
	);
	assert!(asked.contains("2 EL Bratbutter"), "{asked}");
	// with its steps in the JSON-LD, not the text too
	assert!(!asked.contains("The page's text:"), "{asked}");

	// a page without JSON-LD: its text, the model's numbers
	let plain = json!({ "url": "https://blog.example.com/plain" });
	let (status, r) = post(&lea, "/members/import", plain).await;
	assert_eq!(status, 200, "{r}");
	assert_eq!(r["serves"], 3);
	let asked = llm.last_text();
	assert!(
		asked.contains("The page's text:\n<untrusted>\nGrandma's Zopf"),
		"{asked}"
	);

	// what can't be fetched
	let (status, err) = post(
		&lea,
		"/members/import",
		json!({ "url": "http://10.0.0.1/" }),
	)
	.await;
	assert_eq!(status, 400);
	assert!(err["message"].as_str().unwrap().contains("public"), "{err}");
	let (status, err) = post(
		&lea,
		"/members/import",
		json!({ "url": "https://x.ch/gone" }),
	)
	.await;
	assert_eq!(status, 400);
	assert!(err["message"].as_str().unwrap().contains("404"), "{err}");
}

#[tokio::test]
async fn imports_are_limited() {
	let ts = server(Some(fake_llm(answer())));
	let lea = ts.site_user(Some("member"));
	let url = json!({ "url": "https://www.example-milk.ch/sections" });
	for _ in 0..MAX_IMPORTS {
		let (status, r) = post(&lea, "/members/import", url.clone()).await;
		assert_eq!(status, 200, "{r}");
	}
	let (status, err) = post(&lea, "/members/import", url.clone()).await;
	assert_eq!(status, 400);
	assert!(
		err["message"].as_str().unwrap().contains("Too many"),
		"{err}"
	);
	// per user
	let mario = ts.site_user(Some("member"));
	let (status, _) = post(&mario, "/members/import", url).await;
	assert_eq!(status, 200);
}

#[tokio::test]
async fn without_a_key_there_is_no_import() {
	let ts = server(None);
	let lea = ts.site_user(Some("member"));
	let url = json!({ "url": "https://www.example-milk.ch/swissmilk" });
	let (status, err) = post(&lea, "/members/import", url).await;
	assert_eq!(status, 400);
	assert!(
		err["message"].as_str().unwrap().contains("not configured"),
		"{err}"
	);
}

#[tokio::test]
async fn json_ld_without_steps_comes_with_the_text() {
	let llm = fake_llm(answer());
	let html = r#"<script type="application/ld+json">
		{"@type": ["Recipe"], "name": "Zopf", "recipeIngredient": ["500 g Mehl"]}
		</script><main><p>Knead the dough for ten minutes.</p></main>"#;
	let page = Page {
		url: Url::parse("https://example.com/zopf").unwrap(),
		html: html.into(),
	};
	recipe_of(&page, &[], llm.as_ref()).await.unwrap();
	let asked = llm.last_text();
	assert!(asked.contains("500 g Mehl"), "{asked}");
	assert!(asked.contains("Knead the dough"), "{asked}");
}
