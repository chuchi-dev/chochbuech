//! The content model through the queries of the website.

use serde_json::{Value, json};
use zipp::axum::body::{Body, to_bytes};
use zipp::axum::http::{Request, header};
use zipp::entries::api::CreateReq;
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
