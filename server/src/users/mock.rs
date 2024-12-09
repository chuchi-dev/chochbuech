use std::{
	collections::HashMap,
	sync::{Arc, RwLock},
};

use chuchi_postgres::{
	db::Conn,
	time::{DateTime, Timeout},
	Result, UniqueId,
};

use super::data::{
	Auth, Session, Token, User, UsersBuilderTrait, UsersTrait, UsersWithConn,
};

pub struct UsersBuilder {
	inner: Arc<Users>,
}

impl UsersBuilder {
	pub fn new() -> Self {
		Self {
			inner: Arc::new(Users::new()),
		}
	}
}

impl UsersBuilderTrait for UsersBuilder {
	fn with_conn<'a>(&'a self, _conn: Conn<'a>) -> UsersWithConn<'a> {
		Box::new(self.inner.clone())
	}
}

#[derive(Debug, Clone)]
struct UserRaw {
	id: UniqueId,
	name: String,
	email: String,
	auth: Auth,
	created_on: DateTime,
}

impl From<UserRaw> for User {
	fn from(raw: UserRaw) -> Self {
		Self {
			id: raw.id,
			name: raw.name,
			email: raw.email,
			auth: raw.auth,
			created_on: raw.created_on,
		}
	}
}

#[derive(Debug, Clone)]
struct SessionRaw {
	token: Token,
	timeout: Timeout,
	user_id: UniqueId,
	created_on: DateTime,
	oauth_token: Option<String>,
}

impl From<SessionRaw> for Session {
	fn from(raw: SessionRaw) -> Self {
		Self {
			token: raw.token,
			timeout: raw.timeout,
			user_id: raw.user_id,
			created_on: raw.created_on,
			oauth_token: raw.oauth_token,
		}
	}
}

pub struct Users {
	users: RwLock<HashMap<UniqueId, UserRaw>>,
	sessions: RwLock<HashMap<Token, SessionRaw>>,
}

impl Users {
	pub fn new() -> Self {
		Self {
			users: RwLock::new(HashMap::new()),
			sessions: RwLock::new(HashMap::new()),
		}
	}
}

fn into<T>(v: impl Into<T>) -> T {
	v.into()
}

#[async_trait::async_trait]
impl UsersTrait for Arc<Users> {
	async fn by_id(&self, id: &UniqueId) -> Result<Option<User>> {
		let inner = self.users.read().unwrap();
		Ok(inner.get(id).cloned().map(into))
	}

	async fn by_email(&self, email: &str) -> Result<Option<User>> {
		let inner = self.users.read().unwrap();
		Ok(inner.values().find(|u| u.email == email).cloned().map(into))
	}

	async fn create(&self, user: &User) -> Result<()> {
		let mut inner = self.users.write().unwrap();
		let raw = UserRaw {
			id: user.id,
			name: user.name.clone(),
			email: user.email.clone(),
			auth: user.auth.clone(),
			created_on: user.created_on,
		};
		inner.insert(user.id, raw);

		Ok(())
	}

	async fn new_session(
		&self,
		user_id: &UniqueId,
		token: Option<String>,
	) -> Result<Session> {
		let mut inner = self.sessions.write().unwrap();
		let session = Session::new(*user_id, token);
		let raw = SessionRaw {
			token: session.token.clone(),
			timeout: session.timeout.clone(),
			user_id: session.user_id,
			created_on: session.created_on,
			oauth_token: session.oauth_token.clone(),
		};

		inner.insert(session.token.clone(), raw);

		Ok(session)
	}

	async fn session_by_token(&self, token: &Token) -> Result<Option<Session>> {
		let inner = self.sessions.read().unwrap();
		Ok(inner.get(token).cloned().map(into))
	}
}
