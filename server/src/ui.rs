use chuchi::header::RequestHeader;
use std::path::{Path, PathBuf};
use tokio::fs;

use chuchi::fs::{serve_file, Caching, IntoPathBuf};
use chuchi::{get, Chuchi, Error, Request, Resource, Response};
use chuchi_ssr::JsServer;

use serde::{Deserialize, Serialize};

#[get("/{*?remaining}")]
async fn all(
	req: &mut Request,
	ssr: &JsServer,
	dist_dir: &UiPublicDir,
	caching: &UiCaching,
) -> Result<Response, Error> {
	// let's first check if a file exists

	async fn ui_path(
		header: &RequestHeader,
		dist_dir: &PathBuf,
	) -> Option<PathBuf> {
		let req_path = header.uri().path().into_path_buf().ok()?;
		let ui_path = dist_dir.join(req_path);
		let meta = fs::metadata(&ui_path).await.ok()?;

		meta.is_file().then_some(ui_path)
	}

	if let Some(path) = ui_path(req.header(), &dist_dir.0).await {
		serve_file(path, req, Some(caching.0.clone()))
			.await
			.map_err(Error::from_server_error)
	} else {
		ssr.request(req).await.map_err(Error::from_server_error)
	}
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Options {
	ssr_manifest: serde_json::Value,
}

#[derive(Resource)]
struct UiPublicDir(PathBuf);

#[derive(Resource)]
struct UiCaching(Caching);

pub fn routes(dist_dir: String, server: &mut Chuchi) -> JsServer {
	// add ssr
	let index =
		std::fs::read_to_string(format!("{dist_dir}/index.html")).unwrap();
	let manifest =
		std::fs::read_to_string(format!("{dist_dir}/ssr-manifest.json"))
			.unwrap();
	// since we only have two cores it doesn't make sense to run more threads than
	// that
	let js_server = JsServer::new(
		dist_dir.clone(),
		index,
		Options {
			ssr_manifest: serde_json::from_str(&manifest).unwrap(),
		},
		2,
	);
	server.add_resource(js_server.clone());
	let public_dir = Path::new(&dist_dir).join("public");
	server.add_resource(UiPublicDir(public_dir));
	server.add_resource(UiCaching(Caching::default()));
	server.add_route(all);

	js_server
}
