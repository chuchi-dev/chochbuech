//! The images of a member's private recipes for the website: an entry of
//! zipp-members names its images by id. It only answers the images a
//! member uploaded and those of live entries (public anyway, like the
//! photo of a recipe an admin made for them), never another member's
//! private photo. Not GraphQL's `assets(id:)`: crelte's server runs the
//! queries for any visitor with the site's token, which reads every
//! asset, so counting up the ids would give the urls of all private
//! photos (the filename in the url is all that keeps them).

use serde::{Deserialize, Serialize};
use zipp::AppState;
use zipp::assets::get_asset;
use zipp::auth::SiteUser;
use zipp::axum::extract::{Query, State};
use zipp::axum::routing::get;
use zipp::axum::{Json, Router};
use zipp::error::{Error, Result};
use zipp::rusqlite::{Connection, OptionalExtension, params};

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

/// `GET /members/images?ids=4,12`: the images of these ids that exist and
/// are the user's.
async fn images(
	user: SiteUser,
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
		if !is_theirs(&conn, id, user.id)? {
			continue;
		}
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

/// Whether the user uploaded the asset or a live entry shows it. Not
/// "in one of their entries": a member can put any id into their own.
fn is_theirs(conn: &Connection, asset: i64, user: i64) -> Result<bool> {
	let found = conn
		.query_row(
			"SELECT 1 FROM assets a WHERE a.id = ?1 AND (a.uploaded_by = ?2
			OR EXISTS (SELECT 1 FROM asset_relations r
				JOIN entry_sites s ON s.entry_id = r.entry_id AND s.site = r.site
				JOIN entries e ON e.id = r.entry_id
				WHERE r.asset_id = a.id AND s.published = 1
				AND e.deleted_at IS NULL))",
			params![asset, user],
			|_| Ok(()),
		)
		.optional()?;
	Ok(found.is_some())
}
