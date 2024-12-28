use crate::{
	users::{
		self,
		api::{Authenticated, LoginReq},
		data::{Auth, User},
		mock::UsersBuilder,
		Users,
	},
	waitlist::{self, mock::WaitlistBuilder},
	Config, DbConf,
};

use chuchi::api::testing::ChuchiSharedApi;
use chuchi_postgres::{
	db::{Conn, Db},
	time::DateTime,
	UniqueId,
};

async fn init() -> ChuchiSharedApi {
	let cfg = Config {
		tracing: None,
		database: DbConf {
			host: String::new(),
			name: String::new(),
			user: String::new(),
			password: String::new(),
		},
	};

	let mut server = chuchi::build("127.0.0.1:0").await.unwrap();

	server.add_resource(cfg);
	server.add_resource(Db::new_memory());
	server.add_resource::<users::data::Users>(Box::new(UsersBuilder::new()));
	server.add_resource::<waitlist::data::Waitlist>(Box::new(
		WaitlistBuilder::new(),
	));
	// server.add_resource::<recipes::data::Recipes>(Box::new(
	// 	RecipesBuilder::new(),
	// ));

	crate::init(&mut server);

	let chuchi = server.build().await.unwrap();
	ChuchiSharedApi::new(chuchi.shared())
}

async fn init_user(pit: &ChuchiSharedApi) -> Authenticated {
	let users = pit.data().get::<Users>().unwrap();
	let users = users.with_conn(Conn::new_memory());
	let user = User {
		id: UniqueId::new(),
		name: "test".into(),
		email: "test@chochbuech.com".into(),
		auth: Auth::Password(
			bcrypt::hash("test", bcrypt::DEFAULT_COST).unwrap(),
		),
		created_on: DateTime::now(),
	};
	users.create(&user).await.unwrap();

	pit.request(&LoginReq {
		email: "test@chochbuech.com".parse().unwrap(),
		password: "test".into(),
	})
	.await
	.unwrap()
}

#[tokio::test]
async fn test_inits() {
	let pit = init().await;
	let login = init_user(&pit).await;

	assert_eq!(login.user.name, "test");
}
