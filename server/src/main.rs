mod api;
mod cors;
mod ui;

use std::fs;

use chuchi::Resource;
use clap::Parser;
use serde::Deserialize;
use tracing::info;

#[derive(Debug, Parser)]
#[command(version)]
struct Args {
	#[arg(long)]
	tracing: Option<String>,
	#[arg(long, default_value = "./config.toml")]
	config: String,
}

#[derive(Debug, Clone, Deserialize, Resource)]
#[serde(rename_all = "kebab-case")]
struct Config {
	tracing: Option<String>,
}

const DEFAULT_TRACING: &str = "server=info,chuchi=info,warning";

#[cfg(debug_assertions)]
const UI_DIR: &str = "../ui/dist";
#[cfg(not(debug_assertions))]
const UI_DIR: &str = "./ui";

#[tokio::main]
async fn main() {
	let args = Args::parse();

	let cfg_string =
		fs::read_to_string(args.config).expect("failed to read config.toml");
	let cfg: Config =
		toml::from_str(&cfg_string).expect("failed to read config.toml");

	let tracing_cfg = args
		.tracing
		.as_ref()
		.or(cfg.tracing.as_ref())
		.map(|s| s.as_str())
		.unwrap_or(DEFAULT_TRACING);

	tracing_subscriber::fmt()
		.with_env_filter(tracing_cfg)
		.init();

	let mut server = chuchi::build("0.0.0.0:4986")
		.await
		.expect("Address could not be parsed");

	server.add_resource(cfg);

	api::routes(&mut server);
	info!("using ui dir {UI_DIR}");
	let js_server = ui::routes(UI_DIR.to_string(), &mut server);

	if cfg!(debug_assertions) {
		info!("adding cors headers catcher");
		server.add_catcher(cors::CorsHeaders);
	}

	let server = server.build().await.unwrap();
	js_server.route_internally(server.shared()).await;
	server.run().await.unwrap();
}
