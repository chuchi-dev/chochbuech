#[macro_use]
mod utils;
mod api;
mod cors;
mod error;
#[allow(dead_code)]
mod recipes;
#[cfg(test)]
mod tests;
mod ui;
mod users;
mod waitlist;

use std::{fs, path::Path};

use chuchi::{Chuchi, Resource};
use chuchi_postgres::{db::Db, Database};
use clap::Parser;
use serde::Deserialize;
use tracing::info;
use users::data::{Auth, User, UsersBuilderTrait};

#[derive(Debug, Parser)]
#[command(version)]
struct Args {
	#[arg(long)]
	tracing: Option<String>,
	#[arg(long, default_value = "./config.toml")]
	config: String,
	#[command(subcommand)]
	subcmd: Option<SubCommand>,
}

#[derive(Debug, Parser)]
enum SubCommand {
	CreateUser(CreateUser),
}

#[derive(Debug, Parser)]
struct CreateUser {
	email: String,
	name: String,
	password: String,
}

#[derive(Debug, Clone, Deserialize, Resource)]
#[serde(rename_all = "kebab-case")]
struct Config {
	tracing: Option<String>,
	database: DbConf,
}

#[derive(Debug, Clone, Deserialize)]
struct DbConf {
	pub host: String,
	pub name: String,
	pub user: String,
	pub password: String,
}

const DEFAULT_TRACING: &str = "server=info,chuchi=info,warn";

#[cfg(debug_assertions)]
const UI_DIR: &str = "../ui/dist";
#[cfg(not(debug_assertions))]
const UI_DIR: &str = "./ui";

fn init(server: &mut Chuchi) {
	api::routes(server);
	users::routes::routes(server);
	waitlist::routes::routes(server);
}

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

	let db_cfg = &cfg.database;
	let database = Database::with_host(
		&db_cfg.host,
		&db_cfg.name,
		&db_cfg.user,
		&db_cfg.password,
	)
	.await
	.unwrap();
	let db = Db::from(database.clone());
	let conn = db.get().await.unwrap();

	let users = users::database::UsersBuilder::new(&database).await;
	let waitlist = waitlist::database::WaitlistBuilder::new(&database).await;
	// let recipes = recipes::database::RecipesBuilder::new(&database).await;

	match args.subcmd {
		Some(SubCommand::CreateUser(CreateUser {
			email,
			name,
			password,
		})) => {
			let user = User::new(
				name,
				email,
				Auth::Password(
					bcrypt::hash(password, bcrypt::DEFAULT_COST).unwrap(),
				),
			);
			users.with_conn(conn.conn()).create(&user).await.unwrap();
			eprintln!("user created {user:?}");
			return;
		}
		None => {}
	}

	let mut server = chuchi::build("0.0.0.0:4986")
		.await
		.expect("Address could not be parsed");

	server.add_resource(cfg);
	server.add_resource(db);
	server.add_resource::<users::data::Users>(Box::new(users));
	server.add_resource::<waitlist::data::Waitlist>(Box::new(waitlist));
	// server.add_resource::<recipes::data::Recipes>(Box::new(recipes));

	init(&mut server);

	let js_server = if Path::new(UI_DIR).exists() {
		info!("using ui dir {UI_DIR}");
		Some(ui::routes(UI_DIR.to_string(), &mut server))
	} else {
		info!("ui dir {UI_DIR} does not exist, not serving ui");
		None
	};

	if cfg!(debug_assertions) {
		info!("adding cors headers catcher");
		server.add_catcher(cors::CorsHeaders);
	}

	let server = server.build().await.unwrap();
	if let Some(js_server) = js_server {
		js_server.route_internally(server.shared()).await;
	}
	server.run().await.unwrap();
}
