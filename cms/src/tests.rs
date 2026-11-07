//! The content model through the queries of the website.

use serde_json::{Value, json};
use zipp::axum::body::{Body, to_bytes};
use zipp::axum::http::{Request, header};
use zipp::entries::api::{AuthorReq, CreateReq};
use zipp::testing::{Client, TestServer, input};

use crate::content::schema;

const RECIPES: &str = include_str!("../../svelte/src/queries/recipes.graphql");
const ENTRY: &str = include_str!("../../svelte/src/queries/entry.graphql");

// a 1x1 png
const PNG: &[u8] = &[
	0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a, 0x00, 0x00, 0x00, 0x0d,
	0x49, 0x48, 0x44, 0x52, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01,
	0x08, 0x06, 0x00, 0x00, 0x00, 0x1f, 0x15, 0xc4, 0x89, 0x00, 0x00, 0x00,
	0x0d, 0x49, 0x44, 0x41, 0x54, 0x78, 0x9c, 0x63, 0xf8, 0xcf, 0xc0, 0xf0,
	0x1f, 0x00, 0x05, 0x00, 0x01, 0xff, 0x89, 0x99, 0x3d, 0x1d, 0x00, 0x00,
	0x00, 0x00, 0x49, 0x45, 0x4e, 0x44, 0xae, 0x42, 0x60, 0x82,
];

async fn photo(admin: &Client) -> i64 {
	let resp = admin.upload("/assets", &[("dish.png", PNG)]).await;
	let assets: Value = Client::json(resp).await.unwrap();
	assets[0]["id"].as_i64().unwrap()
}

async fn create(
	admin: &Client,
	section: &str,
	ty: &str,
	title: &str,
	mut fields: Value,
) -> i64 {
	if section == "recipes" {
		fields["image"] = json!([photo(admin).await]);
	}
	let mut i = input(section, ty, title, fields);
	i.published = true;
	admin.call(&CreateReq(i)).await.unwrap().id
}

fn steps(paragraphs: &[&str]) -> Value {
	let content: Vec<_> = paragraphs
        .iter()
        .map(|p| json!({ "type": "paragraph", "content": [{ "type": "text", "text": p }] }))
        .collect();
	json!({ "type": "doc", "content": content })
}

#[tokio::test]
async fn the_list_searches_and_filters_by_category() {
	let ts = TestServer::new(schema());
	let admin = ts.admin();
	let mains = create(
		&admin,
		"categories",
		"category",
		"Main Courses",
		json!({ "singular": "Main Course" }),
	)
	.await;
	let drinks = create(
		&admin,
		"categories",
		"category",
		"Drinks",
		json!({ "singular": "Drink" }),
	)
	.await;
	let lea = create(
		&admin,
		"cooks",
		"cook",
		"Léa Linster",
		json!({ "memberSince": 2024 }),
	)
	.await;
	let recipe = |category: i64, cook: Option<i64>| {
		json!({
			"category": [category],
			"cook": cook.map(|c| vec![c]).unwrap_or_default(),
			"serves": 2,
			"chapters": [{ "type": "chapter", "fields": {
				"ingredients": [{ "amount": 2.0, "unit": "piece", "name": "Soles" }],
				"steps": steps(&["Flour the soles.", "Fry them in butter."]),
			} }],
		})
	};
	create(
		&admin,
		"recipes",
		"recipe",
		"Sole Meunière",
		recipe(mains, Some(lea)),
	)
	.await;
	create(
		&admin,
		"recipes",
		"recipe",
		"Spaghetti with pea pesto",
		recipe(mains, None),
	)
	.await;
	create(
		&admin,
		"recipes",
		"recipe",
		"Bee's Knees Highball",
		recipe(drinks, None),
	)
	.await;

	let titles = |data: &Value| -> Vec<String> {
		data["data"]["recipes"]
			.as_array()
			.unwrap()
			.iter()
			.map(|r| r["title"].as_str().unwrap().to_string())
			.collect()
	};
	let all = ts
		.graphql(RECIPES, json!({ "limit": 12, "offset": 0 }))
		.await;
	assert_eq!(all["data"]["total"], 3);
	let mains_only = ts
		.graphql(
			RECIPES,
			json!({ "category": [mains.to_string()], "limit": 12 }),
		)
		.await;
	assert_eq!(mains_only["data"]["total"], 2);
	let found = ts.graphql(RECIPES, json!({ "search": "meuni" })).await;
	assert_eq!(titles(&found), ["Sole Meunière"]);
	let by_lea = ts
		.graphql(RECIPES, json!({ "cook": [lea.to_string()] }))
		.await;
	assert_eq!(titles(&by_lea), ["Sole Meunière"]);
	assert_eq!(
		by_lea["data"]["recipes"][0]["category"][0]["singular"],
		"Main Course"
	);
}

#[tokio::test]
async fn a_recipe_page_has_its_chapters() {
	let ts = TestServer::new(schema());
	let admin = ts.admin();
	let mains = create(
		&admin,
		"categories",
		"category",
		"Main Courses",
		json!({ "singular": "Main Course" }),
	)
	.await;
	let fields = json!({
		"category": [mains],
		"serves": 4,
		"chapters": [
			{ "type": "chapter", "fields": {
				"ingredients": [
					{ "amount": 2.5, "unit": "dl", "name": "Red wine vinegar" },
					{ "name": "Salt" },
				],
				"steps": steps(&["Tie the tournedos.", "Peel the potatoes."]),
			} },
			{ "type": "chapter", "fields": { "title": "Arranging", "steps": steps(&["Serve."]) } },
		],
	});
	create(&admin, "recipes", "recipe", "Beef fillet", fields).await;

	let data = ts
		.graphql(
			ENTRY,
			json!({ "uri": ["recipes/beef-fillet"], "siteId": [1] }),
		)
		.await;
	let entry = &data["data"]["entry"];
	assert_eq!(entry["serves"], 4.0);
	let chapters = entry["chapters"].as_array().unwrap();
	assert_eq!(chapters.len(), 2);
	assert_eq!(chapters[0]["ingredients"][0]["unit"], "dl");
	assert_eq!(chapters[0]["ingredients"][1]["amount"], Value::Null);
	let html = chapters[0]["steps"]["chunks"][0]["html"].as_str().unwrap();
	assert_eq!(html, "<p>Tie the tournedos.</p><p>Peel the potatoes.</p>");
	assert_eq!(chapters[1]["title"], "Arranging");
}

/// A POST of the website's frontend, its status and the json answer.
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

#[tokio::test]
async fn a_member_logs_in_on_the_website_but_not_in_the_studio() {
	let ts = TestServer::new(schema());
	// member0@test.ch with the password `password`
	ts.user(Some("member"));
	let login = json!({ "email": "member0@test.ch", "password": "password" });

	let wrong = json!({ "email": "member0@test.ch", "password": "nope" });
	let (status, _) = post(&ts.visitor(None), "/site-auth/login", wrong).await;
	assert_eq!(status, 401);

	let (status, auth) =
		post(&ts.visitor(None), "/site-auth/login", login.clone()).await;
	assert_eq!(status, 200, "{auth}");
	assert_eq!(auth["user"]["role"], "member");
	let token = auth["session"]["token"].as_str().unwrap();
	let me = ts.visitor(Some(token)).get("/site-auth/me").await;
	assert_eq!(me.status().as_u16(), 200);

	// the studio refuses the password and the site's token
	let (status, err) = post(&ts.anonymous(), "/api/auth/login", login).await;
	assert_ne!(status, 200);
	assert!(err.to_string().contains("website"), "{err}");
	let session = ts.with_token(token).get("/api/auth/session").await;
	assert_ne!(session.status().as_u16(), 200);
}

const COOK_OF: &str = include_str!("../../svelte/src/queries/cookOf.graphql");
const FOR_EDIT: &str =
	include_str!("../../svelte/src/queries/recipeForEdit.graphql");

/// A request of the website's editor (`/members/*` of the crelte
/// server), its status and the json answer.
async fn send(
	client: &Client,
	method: &str,
	path: &str,
	body: Value,
) -> (u16, Value) {
	let req = Request::builder()
		.method(method)
		.uri(path)
		.header(header::CONTENT_TYPE, "application/json")
		.body(Body::from(body.to_string()))
		.unwrap();
	let resp = client.send(req).await;
	let status = resp.status().as_u16();
	let body = to_bytes(resp.into_body(), usize::MAX).await.unwrap();
	(status, serde_json::from_slice(&body).unwrap_or_default())
}

#[tokio::test]
async fn members_write_their_own_recipes() {
	let ts = TestServer::new(schema());
	let admin = ts.admin();
	let mains = create(
		&admin,
		"categories",
		"category",
		"Main Courses",
		json!({ "singular": "Main Course" }),
	)
	.await;
	let lea = ts.site_user(Some("member"));
	let (_, me) = send(&lea, "GET", "/site-auth/me", Value::Null).await;
	let lea_id = me["user"]["id"].as_i64().unwrap();
	// her public profile, she is its author
	let cook = create(&admin, "cooks", "cook", "Léa Linster", json!({})).await;
	admin
		.call(&AuthorReq {
			id: cook,
			author_id: Some(lea_id),
		})
		.await
		.unwrap();
	let found = ts
		.graphql(COOK_OF, json!({ "author": [lea_id], "siteId": [1] }))
		.await;
	assert_eq!(found["data"]["cook"]["id"], cook.to_string(), "{found}");

	// the editor: an image, then the recipe as the website sends it
	let resp = lea.upload("/members/assets", &[("dish.png", PNG)]).await;
	let image: Value = Client::json(resp).await.unwrap();
	let image = image["id"].as_i64().unwrap();
	let fields = json!({
		"category": [mains],
		"cook": [cook],
		"image": [image],
		"serves": 2,
		"totalTime": 40,
		"activeTime": 20,
		"chapters": [{ "type": "chapter", "fields": {
			"title": "",
			"ingredients": [
				{ "amount": 2, "unit": "piece", "name": "Soles" },
				{ "amount": null, "unit": "g", "name": "Butter" },
			],
			"steps": steps(&["Flour the soles.", "Fry them in butter."]),
		} }],
	});
	let new = json!({
		"section": "recipes",
		"type": "recipe",
		"title": "Sole Meunière",
		"fields": fields,
		"published": true,
	});
	let (status, entry) = send(&lea, "POST", "/members/entries", new).await;
	assert_eq!(status, 200, "{entry}");
	assert_eq!(entry["published"], true);
	assert_eq!(entry["uri"], "recipes/sole-meunière");
	let id = entry["id"].as_i64().unwrap();

	// live at once, by her
	let list = ts
		.graphql(RECIPES, json!({ "cook": [cook.to_string()] }))
		.await;
	assert_eq!(list["data"]["recipes"][0]["title"], "Sole Meunière");
	let page = ts
		.graphql(ENTRY, json!({ "uri": [entry["uri"]], "siteId": [1] }))
		.await;
	assert_eq!(page["data"]["entry"]["authorId"], lea_id.to_string());
	let copy = ts
		.graphql(FOR_EDIT, json!({ "id": [id], "siteId": [1] }))
		.await;
	assert_eq!(copy["data"]["recipe"]["image"][0]["id"], image.to_string());

	// a change replaces what it sends
	let change = json!({ "title": "Sole Meunière", "fields": { "serves": 4 } });
	let path = format!("/members/entries/{id}");
	let (status, saved) = send(&lea, "PUT", &path, change.clone()).await;
	assert_eq!((status, &saved["saved"]), (200, &json!("live")), "{saved}");
	assert_eq!(saved["entry"]["fields"]["cook"], json!([cook]));

	// invalid fields come back with their paths
	let bad = json!({ "title": "Sole", "fields": { "serves": 0 } });
	let (status, err) = send(&lea, "PUT", &path, bad).await;
	assert_eq!(status, 400, "{err}");
	assert_eq!(err["error"]["type"], "INVALID", "{err}");
	assert_eq!(err["error"]["detail"][0]["path"], "serves", "{err}");

	// another member can't read or change it
	let mario = ts.site_user(Some("member"));
	let (status, _) = send(&mario, "GET", &path, Value::Null).await;
	assert_eq!(status, 403);
	let (status, _) = send(&mario, "PUT", &path, change).await;
	assert_eq!(status, 403);
	let (status, _) = send(&mario, "DELETE", &path, Value::Null).await;
	assert_eq!(status, 403);

	// an admin on the website writes their own, without a cook: Anonymous
	let admin_site = ts.site_user(None);
	let mut fields = fields;
	fields["cook"] = json!([]);
	fields["image"] = json!([photo(&admin).await]);
	let new = json!({
		"section": "recipes",
		"type": "recipe",
		"title": "Rösti",
		"fields": fields,
		"published": true,
	});
	let (status, rosti) =
		send(&admin_site, "POST", "/members/entries", new).await;
	assert_eq!(status, 200, "{rosti}");
	let rosti_path = format!("/members/entries/{}", rosti["id"]);
	let change =
		json!({ "title": "Rösti with egg", "fields": { "serves": 2 } });
	let (status, saved) = send(&admin_site, "PUT", &rosti_path, change).await;
	assert_eq!((status, &saved["saved"]), (200, &json!("live")), "{saved}");
	let (_, mine) = send(
		&admin_site,
		"GET",
		"/members/entries?section=recipes",
		Value::Null,
	)
	.await;
	assert_eq!(mine["entries"][0]["title"], "Rösti with egg", "{mine}");
	assert_eq!(mine["total"], 1);

	let (status, _) = send(&lea, "DELETE", &path, Value::Null).await;
	assert_eq!(status, 200);
	let list = ts.graphql(RECIPES, json!({})).await;
	assert_eq!(list["data"]["total"], 1);
	assert_eq!(list["data"]["recipes"][0]["cook"], json!([]));
}
