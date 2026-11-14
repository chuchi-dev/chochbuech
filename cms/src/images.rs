//! The images of a member's private recipes for the website: an entry of
//! zipp-members names its images by id, GraphQL only has the images of
//! live entries and no asset by id. The files are public by their url, so
//! this only answers what the editor's upload answers.

use serde::{Deserialize, Serialize};
use zipp::AppState;
use zipp::assets::get_asset;
use zipp::auth::SiteUser;
use zipp::axum::extract::{Query, State};
use zipp::axum::routing::get;
use zipp::axum::{Json, Router};
use zipp::error::{Error, Result};

/// Images of one request, at most: the photo and the step images of a
/// recipe.
const MAX_IDS: usize = 100;

pub fn routes() -> Router<AppState> {
	Router::new().route("/members/images", get(images))
}

#[derive(Debug, Deserialize)]
struct ImagesReq {
	/// `4,12,13`
	ids: String,
}

/// Like the answer of an upload of zipp-members.
#[derive(Debug, Serialize)]
struct Image {
	id: i64,
	url: String,
	width: Option<u32>,
	height: Option<u32>,
}

/// `GET /members/images?ids=4,12`: the images of these ids that exist;
/// for a logged in user, like the editor's uploads.
async fn images(
	_user: SiteUser,
	State(state): State<AppState>,
	Query(req): Query<ImagesReq>,
) -> Result<Json<Vec<Image>>> {
	let ids: Vec<i64> = req
		.ids
		.split(',')
		.filter(|s| !s.is_empty())
		.map(|s| s.trim().parse())
		.collect::<std::result::Result<_, _>>()
		.map_err(|_| Error::Request("ids are numbers".into()))?;
	if ids.len() > MAX_IDS {
		return Err(Error::Request("Too many images".into()));
	}
	let base = state.public_url.trim_end_matches('/');
	let conn = state.db.conn();
	let mut out = vec![];
	for id in ids {
		let a = match get_asset(&conn, id) {
			Ok(a) => a,
			Err(Error::NotFound) => continue,
			Err(e) => return Err(e),
		};
		if a.kind != "image" {
			continue;
		}
		out.push(Image {
			id: a.id,
			url: format!("{base}{}", a.url(None)),
			width: a.width,
			height: a.height,
		});
	}
	Ok(Json(out))
}
