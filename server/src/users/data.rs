use std::time::Duration;

use chuchi::impl_res_extractor;
use chuchi_postgres::{
	db::Conn,
	time::{DateTime, Timeout},
	Result, UniqueId,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct User {
	pub id: UniqueId,
	pub name: String,
	pub email: String,
	pub auth: Auth,
	pub created_on: DateTime,
}

impl User {
	pub fn new(name: String, email: String, auth: Auth) -> Self {
		Self {
			id: UniqueId::new(),
			name,
			email,
			auth,
			created_on: DateTime::now(),
		}
	}
}

pub type Token = chuchi_crypto::token::Token<32>;

#[derive(Debug, Clone)]
pub enum Auth {
	Password(String),
	OAuth {
		platform: OAuthPlatform,
		oauth_id: String,
	},
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct ShortUser {
	pub id: UniqueId,
	pub oauth: Option<OAuthPlatform>,
	pub name: String,
	pub email: String,
	pub created_on: DateTime,
}

impl From<User> for ShortUser {
	fn from(user: User) -> Self {
		Self {
			id: user.id,
			name: user.name,
			email: user.email,
			created_on: user.created_on,
			oauth: match user.auth {
				Auth::OAuth { platform, .. } => Some(platform),
				_ => None,
			},
		}
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OAuthPlatform {
	Github,
}

// 30days
const SESSION_TIMEOUT: Duration = Duration::from_secs(30 * 24 * 60 * 60);
// 10 days remaining
const RENEW_AFTER: Duration = Duration::from_secs(10 * 24 * 60 * 60);

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Session {
	pub token: Token,
	pub timeout: Timeout,
	pub user_id: UniqueId,
	pub created_on: DateTime,
	pub oauth_token: Option<String>,
}

impl Session {
	pub fn new(user_id: UniqueId, token: Option<String>) -> Self {
		Self {
			token: Token::new(),
			timeout: Timeout::new(SESSION_TIMEOUT),
			user_id,
			created_on: DateTime::now(),
			oauth_token: token,
		}
	}

	pub fn to_short(&self) -> ShortSession {
		ShortSession {
			token: self.token.clone(),
			timeout: self.timeout.clone(),
		}
	}

	pub fn is_valid(&self) -> bool {
		!self.timeout.has_elapsed()
	}

	// does not check if the time is valid
	pub fn should_renew(&self) -> bool {
		self.timeout
			.remaining()
			.map(|d| d < RENEW_AFTER)
			.unwrap_or(true)
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShortSession {
	pub token: Token,
	pub timeout: Timeout,
}

impl From<Session> for ShortSession {
	fn from(s: Session) -> Self {
		Self {
			token: s.token,
			timeout: s.timeout,
		}
	}
}

pub type Users = Box<dyn UsersBuilderTrait + Send + Sync>;
pub type UsersWithConn<'a> = Box<dyn UsersTrait + Send + Sync + 'a>;

impl_res_extractor!(Users);

pub trait UsersBuilderTrait {
	fn with_conn<'a>(&'a self, conn: Conn<'a>) -> UsersWithConn<'a>;
}

#[async_trait::async_trait]
pub trait UsersTrait {
	async fn by_id(&self, id: &UniqueId) -> Result<Option<User>>;

	async fn by_email(&self, email: &str) -> Result<Option<User>>;

	async fn create(&self, user: &User) -> Result<()>;

	async fn new_session(
		&self,
		user_id: &UniqueId,
		token: Option<String>,
	) -> Result<Session>;

	async fn session_by_token(&self, token: &Token) -> Result<Option<Session>>;
}
