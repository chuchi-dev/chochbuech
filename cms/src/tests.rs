//! The content model through the queries of the website.

use serde_json::{Value, json};
use zipp::axum::body::{Body, to_bytes};
use zipp::axum::http::{Request, header};
use zipp::entries::api::{AuthorReq, CreateReq};
use zipp::testing::{Client, TestServer, input};

use crate::content::schema;

const RECIPES: &str = include_str!("../../svelte/src/queries/recipes.graphql");
const ENTRY: &str = include_str!("../../svelte/src/queries/entry.graphql");
const COOK: &str = include_str!("../../svelte/src/queries/cook.graphql");
const GLOBAL: &str = include_str!("../../svelte/src/queries/global.graphql");

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

/// A member logged in on the website with their name, and their id.
async fn member(ts: &TestServer, name: &str) -> (Client, i64) {
	let client = ts.site_user(Some("member"));
	let (status, me) =
		send(&client, "PUT", "/site-auth/me", json!({ "name": name })).await;
	assert_eq!(status, 200, "{me}");
	(client, me["id"].as_i64().unwrap())
}

/// Makes `author` the author of the entry, `None` Anonymous.
async fn set_author(admin: &Client, id: i64, author: Option<i64>) {
	let req = AuthorReq {
		id,
		author_id: author,
	};
	admin.call(&req).await.unwrap();
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
	let (_, lea) = member(&ts, "Léa Linster").await;
	let recipe = |category: i64| {
		json!({
			"category": [category],
			"serves": 2,
			"chapters": [{ "type": "chapter", "fields": {
				"ingredients": [{ "amount": 2.0, "unit": "piece", "name": "Soles" }],
				"steps": steps(&["Flour the soles.", "Fry them in butter."]),
			} }],
		})
	};
	let sole =
		create(&admin, "recipes", "recipe", "Sole Meunière", recipe(mains))
			.await;
	set_author(&admin, sole, Some(lea)).await;
	let title = "Spaghetti with pea pesto";
	let pasta = create(&admin, "recipes", "recipe", title, recipe(mains)).await;
	set_author(&admin, pasta, None).await;
	let title = "Bee's Knees Highball";
	let drink =
		create(&admin, "recipes", "recipe", title, recipe(drinks)).await;
	set_author(&admin, drink, None).await;

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
		.graphql(RECIPES, json!({ "author": [lea.to_string()] }))
		.await;
	assert_eq!(titles(&by_lea), ["Sole Meunière"]);
	let card = &by_lea["data"]["recipes"][0];
	assert_eq!(card["category"][0]["singular"], "Main Course");
	// the card's "by" links to her page, Anonymous without an author
	assert_eq!(card["author"]["name"], "Léa Linster");
	let url = card["author"]["url"].as_str().unwrap();
	assert!(url.ends_with("/cooks/lea-linster"), "{url}");
	let drinks_only = ts
		.graphql(RECIPES, json!({ "category": [drinks.to_string()] }))
		.await;
	assert_eq!(drinks_only["data"]["recipes"][0]["author"], Value::Null);
}

#[tokio::test]
async fn only_categories_with_a_live_recipe_show() {
	let ts = TestServer::new(schema());
	let admin = ts.admin();
	let mut ids = vec![];
	for title in ["Appetisers", "Main Courses", "Desserts"] {
		let fields = json!({ "singular": title });
		ids.push(create(&admin, "categories", "category", title, fields).await);
	}
	let fields = json!({ "category": [ids[0]], "serves": 2 });
	create(&admin, "recipes", "recipe", "Beetroot salad", fields).await;
	// a draft doesn't count
	let image = photo(&admin).await;
	let fields = json!({ "category": [ids[1]], "serves": 2, "image": [image] });
	let mut draft = input("recipes", "recipe", "Gratin", fields);
	draft.published = false;
	admin.call(&CreateReq(draft)).await.unwrap();

	let data = ts.graphql(GLOBAL, json!({ "siteId": [1] })).await;
	assert_eq!(data["data"]["categories"].as_array().unwrap().len(), 3);
	assert_eq!(
		data["data"]["liveCategories"],
		json!([{ "id": ids[0].to_string() }]),
		"{data}"
	);
}

#[tokio::test]
async fn the_cooks_are_the_members_with_a_live_recipe() {
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
	let (lea, lea_id) = member(&ts, "Léa Linster").await;
	let (_, mario_id) = member(&ts, "Mario Bernasconi").await;
	let fields = json!({ "category": [mains], "serves": 2 });
	let sole = create(&admin, "recipes", "recipe", "Sole", fields).await;
	set_author(&admin, sole, Some(lea_id)).await;

	// she edits her profile on the website
	let profile =
		json!({ "fields": { "bio": "Fish on Fridays.", "memberSince": 2024 } });
	let (status, me) = send(&lea, "PUT", "/site-auth/me", profile).await;
	assert_eq!(status, 200, "{me}");
	assert_eq!(me["slug"], "lea-linster");
	assert_eq!(me["name"], "Léa Linster");
	// the slug is an admin's
	let slug = json!({ "slug": "lea" });
	let (status, _) = send(&lea, "PUT", "/site-auth/me", slug).await;
	assert_eq!(status, 400);

	let vars = json!({ "slug": "lea-linster", "siteId": [1] });
	let page = ts.graphql(COOK, vars).await;
	let cook = &page["data"]["cook"];
	assert_eq!(cook["id"], lea_id.to_string(), "{page}");
	assert_eq!(cook["bio"], "Fish on Fridays.");
	assert_eq!(cook["memberSince"], 2024.0);
	let users = ts
		.graphql(
			"{ users(hasEntries: { section: \"recipes\" }) { id } userCount }",
			json!({}),
		)
		.await;
	assert_eq!(
		users["data"]["users"],
		json!([{ "id": lea_id.to_string() }])
	);
	assert_eq!(users["data"]["userCount"], 1);

	// Mario has no recipe: no page
	let page = ts
		.graphql(COOK, json!({ "slug": "mario-bernasconi", "siteId": [1] }))
		.await;
	assert_eq!(page["data"]["cook"], Value::Null, "{page}");
	let by_id = format!("{{ user(id: {mario_id}) {{ name }} }}");
	let user = ts.graphql(&by_id, json!({})).await;
	assert_eq!(user["data"]["user"], Value::Null);

	// neither has Léa once her recipe is gone
	let path = format!("/members/entries/{sole}");
	let (status, _) = send(&lea, "DELETE", &path, Value::Null).await;
	assert_eq!(status, 200);
	let page = ts
		.graphql(COOK, json!({ "slug": "lea-linster", "siteId": [1] }))
		.await;
	assert_eq!(page["data"]["cook"], Value::Null, "{page}");
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
		"source": { "type": "url", "value": "https://www.example.ch/rezepte/filet" },
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
	assert_eq!(
		entry["source"]["url"],
		"https://www.example.ch/rezepte/filet"
	);
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
	let (lea, lea_id) = member(&ts, "Léa Linster").await;

	// the editor: an image, then the recipe as the website sends it
	let resp = lea.upload("/members/assets", &[("dish.png", PNG)]).await;
	let image: Value = Client::json(resp).await.unwrap();
	let image = image["id"].as_i64().unwrap();
	let fields = json!({
		"category": [mains],
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
	assert_eq!(entry["uri"], "recipes/sole-meuniere");
	let id = entry["id"].as_i64().unwrap();

	// live at once, by her
	let list = ts
		.graphql(RECIPES, json!({ "author": [lea_id.to_string()] }))
		.await;
	assert_eq!(list["data"]["recipes"][0]["title"], "Sole Meunière");
	let page = ts
		.graphql(ENTRY, json!({ "uri": [entry["uri"]], "siteId": [1] }))
		.await;
	let recipe = &page["data"]["entry"];
	assert_eq!(recipe["authorId"], lea_id.to_string());
	assert_eq!(recipe["author"]["name"], "Léa Linster");
	let copy = ts
		.graphql(FOR_EDIT, json!({ "id": [id], "siteId": [1] }))
		.await;
	assert_eq!(copy["data"]["recipe"]["image"][0]["id"], image.to_string());

	// a change replaces what it sends
	let change = json!({ "title": "Sole Meunière", "fields": { "serves": 4 } });
	let path = format!("/members/entries/{id}");
	let (status, saved) = send(&lea, "PUT", &path, change.clone()).await;
	assert_eq!((status, &saved["saved"]), (200, &json!("live")), "{saved}");

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

	// an admin on the website writes their own, by them
	let admin_site = ts.site_user(None);
	let mut fields = fields;
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
	assert_eq!(list["data"]["recipes"][0]["title"], "Rösti with egg");
}

#[tokio::test]
async fn private_recipes_are_only_their_authors() {
	let ts = TestServer::new(schema()).routes(crate::images::routes());
	let mains = create(
		&ts.admin(),
		"categories",
		"category",
		"Main Courses",
		json!({ "singular": "Main Course" }),
	)
	.await;
	let (lea, _) = member(&ts, "Léa Linster").await;
	// private: without a photo and a category, like an import
	let new = json!({
		"section": "recipes",
		"type": "recipe",
		"title": "Rösti",
		"fields": {
			"serves": 2,
			"chapters": [{ "type": "chapter", "fields": {
				"ingredients": [{ "amount": 600, "unit": "g", "name": "Potatoes" }],
				"steps": steps(&["Grate the potatoes."]),
			} }],
		},
		"published": false,
	});
	let (status, entry) = send(&lea, "POST", "/members/entries", new).await;
	assert_eq!(status, 200, "{entry}");
	assert_eq!(entry["published"], false);
	let path = format!("/members/entries/{}", entry["id"]);

	// not on the website, but in their list
	let list = ts.graphql(RECIPES, json!({})).await;
	assert_eq!(list["data"]["total"], 0, "{list}");
	let (_, mine) =
		send(&lea, "GET", "/members/entries?section=recipes", Value::Null)
			.await;
	assert_eq!(mine["entries"][0]["published"], false, "{mine}");
	let mario = ts.site_user(Some("member"));
	let (status, _) = send(&mario, "GET", &path, Value::Null).await;
	assert_eq!(status, 403);

	// publishing needs the photo
	let publish = json!({ "title": "Rösti", "fields": {}, "published": true });
	let (status, err) = send(&lea, "PUT", &path, publish.clone()).await;
	assert_eq!(status, 400, "{err}");
	let paths: Vec<_> = err["error"]["detail"]
		.as_array()
		.unwrap()
		.iter()
		.map(|e| e["path"].as_str().unwrap())
		.collect();
	assert!(paths.contains(&"image"), "{err}");

	// their own photo, still private; the private page gets it by id
	let resp = lea.upload("/members/assets", &[("dish.png", PNG)]).await;
	let image: Value = Client::json(resp).await.unwrap();
	let image = image["id"].as_i64().unwrap();
	let fields = json!({ "category": [mains], "image": [image] });
	let change =
		json!({ "title": "Rösti", "fields": fields, "published": false });
	let (status, saved) = send(&lea, "PUT", &path, change).await;
	assert_eq!(status, 200, "{saved}");
	assert_eq!(saved["entry"]["published"], false, "{saved}");
	let images = format!("/members/images?ids={image},999999");
	let resp = lea.get(&images).await;
	let found: Value = Client::json(resp).await.unwrap();
	assert_eq!(found.as_array().unwrap().len(), 1, "{found}");
	assert_eq!(found[0]["id"], image);
	assert!(
		found[0]["url"].as_str().unwrap().contains("/assets/"),
		"{found}"
	);
	assert_eq!(ts.visitor(None).get(&images).await.status().as_u16(), 401);
	// nobody else gets the photo of a private recipe, nor an image of no
	// live entry
	let resp = mario.get(&images).await;
	let found: Value = Client::json(resp).await.unwrap();
	assert_eq!(found, json!([]));
	let unused = photo(&ts.admin()).await;
	let resp = lea.get(&format!("/members/images?ids={unused}")).await;
	let found: Value = Client::json(resp).await.unwrap();
	assert_eq!(found, json!([]));

	// published, then private again
	let (status, saved) = send(&lea, "PUT", &path, publish).await;
	assert_eq!((status, &saved["saved"]), (200, &json!("live")), "{saved}");
	assert_eq!(ts.graphql(RECIPES, json!({})).await["data"]["total"], 1);
	let private = json!({ "title": "Rösti", "fields": {}, "published": false });
	let (status, saved) = send(&lea, "PUT", &path, private).await;
	assert_eq!(status, 200, "{saved}");
	assert_eq!(saved["entry"]["published"], false, "{saved}");
	assert_eq!(ts.graphql(RECIPES, json!({})).await["data"]["total"], 0);
}

#[tokio::test]
async fn a_used_title_gets_a_number_and_photos_stay_their_own() {
	let ts = TestServer::new(schema());
	let mains = create(
		&ts.admin(),
		"categories",
		"category",
		"Main Courses",
		json!({ "singular": "Main Course" }),
	)
	.await;
	let (lea, _) = member(&ts, "Léa Linster").await;
	let (mario, _) = member(&ts, "Mario Bernasconi").await;
	// a copy or an import keeps the title of a recipe that exists, here
	// a private one of someone else: it saves, and says nothing of it
	let rosti = |published| {
		json!({
			"section": "recipes",
			"type": "recipe",
			"title": "Rösti",
			"fields": { "serves": 2 },
			"published": published,
		})
	};
	let (status, first) =
		send(&mario, "POST", "/members/entries", rosti(false)).await;
	assert_eq!(status, 200, "{first}");
	assert_eq!(first["slug"], "roesti");
	let (status, copy) =
		send(&lea, "POST", "/members/entries", rosti(false)).await;
	assert_eq!(status, 200, "{copy}");
	assert_eq!(copy["slug"], "roesti-2");
	assert_eq!(copy["published"], false);
	// a change keeps its slug
	let path = format!("/members/entries/{}", copy["id"]);
	let change = json!({ "title": "Rösti", "fields": { "serves": 3 } });
	let (status, saved) = send(&lea, "PUT", &path, change).await;
	assert_eq!(status, 200, "{saved}");
	assert_eq!(saved["entry"]["slug"], "roesti-2", "{saved}");

	// Mario's private photo isn't Léa's to show
	let resp = mario.upload("/members/assets", &[("dish.png", PNG)]).await;
	let theirs: Value = Client::json(resp).await.unwrap();
	let fields = json!({ "category": [mains], "image": [theirs["id"]] });
	let publish =
		json!({ "title": "Rösti", "fields": fields, "published": true });
	let (status, err) = send(&lea, "PUT", &path, publish).await;
	assert_eq!(status, 400, "{err}");
	assert_eq!(err["error"]["detail"][0]["path"], "image", "{err}");
	let message = err["error"]["detail"][0]["message"].as_str().unwrap();
	assert!(message.contains("neither yours nor public"), "{err}");
	assert_eq!(ts.graphql(RECIPES, json!({})).await["data"]["total"], 0);
	// her own is
	let resp = lea.upload("/members/assets", &[("dish.png", PNG)]).await;
	let hers: Value = Client::json(resp).await.unwrap();
	let fields = json!({ "category": [mains], "image": [hers["id"]] });
	let publish =
		json!({ "title": "Rösti", "fields": fields, "published": true });
	let (status, saved) = send(&lea, "PUT", &path, publish).await;
	assert_eq!((status, &saved["saved"]), (200, &json!("live")), "{saved}");
	let list = ts.graphql(RECIPES, json!({})).await;
	assert_eq!(list["data"]["total"], 1, "{list}");
}
