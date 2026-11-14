//! Importing a recipe from another web site into the website's editor:
//! `POST /members/import {url}` fetches that one page (`fetch`), reads
//! its recipe (`extract`: the schema.org JSON-LD, else the text) and has
//! the recipe assistant write it as a recipe of the editor, in English
//! and in its own words. Nothing is saved: the member edits it and saves
//! it, privately or published with their own photo.

mod extract;
mod fetch;
#[cfg(test)]
mod tests;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use serde::Deserialize;
use zipp::AppState;
use zipp::auth::SiteUser;
use zipp::axum::extract::State;
use zipp::axum::routing::post;
use zipp::axum::{Extension, Json, Router};
use zipp::db::{MINUTE_MS, now};
use zipp::error::{Error, Result};

use crate::assistant::{self, Category, Llm, Recipe};
pub use fetch::{Page, Pages, Web};

/// Imports of a user in [`WINDOW_MS`] before they wait: each one costs a
/// call of the model.
pub const MAX_IMPORTS: usize = 20;
const WINDOW_MS: i64 = 60 * MINUTE_MS;

pub struct Import {
	pages: Arc<dyn Pages>,
	/// `None` without a key: the route says import isn't configured
	llm: Option<Arc<dyn Llm>>,
	tries: Mutex<HashMap<i64, Vec<i64>>>,
}

impl Import {
	pub fn new(pages: Arc<dyn Pages>, llm: Option<Arc<dyn Llm>>) -> Self {
		Self {
			pages,
			llm,
			tries: Mutex::default(),
		}
	}

	/// The web and the assistant of the environment
	/// ([`assistant::from_env`]); without a key there is no import.
	pub fn from_env() -> Self {
		Self::new(Arc::new(Web), assistant::from_env())
	}

	/// Whether the user may import now; counts it.
	fn may_import(&self, user: i64) -> Result<()> {
		let now = now();
		let mut tries = self.tries.lock().unwrap_or_else(|e| e.into_inner());
		tries.retain(|_, times| {
			times.retain(|t| now - *t < WINDOW_MS);
			!times.is_empty()
		});
		let times = tries.entry(user).or_default();
		if times.len() >= MAX_IMPORTS {
			return Err(Error::Request(
				"Too many imports, please try again in an hour.".into(),
			));
		}
		times.push(now);
		Ok(())
	}
}

/// The website's routes of the import (crelte's server forwards
/// `/members/import` with the session's token).
pub fn routes(import: Import) -> Router<AppState> {
	Router::new()
		.route("/members/import", post(import_recipe))
		.layer(Extension(Arc::new(import)))
}

#[derive(Debug, Deserialize)]
struct ImportReq {
	url: String,
}

async fn import_recipe(
	user: SiteUser,
	State(state): State<AppState>,
	Extension(import): Extension<Arc<Import>>,
	Json(req): Json<ImportReq>,
) -> Result<Json<Recipe>> {
	let Some(llm) = import.llm.clone() else {
		return Err(Error::Request("The import is not configured.".into()));
	};
	let url = fetch::parse_url(&req.url).map_err(Error::Request)?;
	import.may_import(user.id)?;
	let page = import.pages.fetch(url).await.map_err(Error::Request)?;
	let categories = assistant::categories(&state)?;
	recipe_of(&page, &categories, llm.as_ref()).await.map(Json)
}

const TASK: &str = "\
Write the recipe of the other site's page below as a new recipe. Write the \
steps in your own words: never copy a sentence of the page, rephrase \
everything. If the page has no recipe, answer noRecipe.";

/// The assistant's recipe of a page; its JSON-LD values win where it has
/// them.
pub async fn recipe_of(
	page: &Page,
	categories: &[Category],
	llm: &dyn Llm,
) -> Result<Recipe> {
	let source = extract::json_ld_recipe(&page.html);
	let input = match &source {
		Some(s) => {
			let json = serde_json::to_string_pretty(s).unwrap_or_default();
			let json = extract::truncate(&json, extract::MAX_TEXT * 2);
			assistant::untrusted("The page's schema.org Recipe as JSON:", json)
		}
		None => {
			let text = extract::main_text(&page.html);
			if text.trim().is_empty() {
				return Err(assistant::no_recipe());
			}
			assistant::untrusted("The page's text:", &text)
		}
	};
	let mut recipe =
		assistant::ask(llm, TASK, None, vec![input], categories).await?;
	if let Some(s) = source {
		recipe.serves = s.serves.or(recipe.serves);
		recipe.total_time = s.total_time.or(recipe.total_time);
		recipe.active_time = s.prep_time.or(recipe.active_time);
	}
	recipe.source = Some(page.url.to_string());
	Ok(recipe)
}
