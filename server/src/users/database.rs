use chuchi_postgres::{
	db::Conn,
	filter,
	table::{table::TableWithConn, Table},
	time::{DateTime, Timeout},
	Connection, Database, FromRow, Result, ToRow, UniqueId,
};

use super::data::{
	self, Auth, OAuthPlatform, Session, Token, User, UsersBuilderTrait,
	UsersTrait,
};

const MIGRATIONS: &[(&str, &str)] = migration_files!("create-users");

#[derive(Debug, Clone)]
pub struct UsersBuilder {
	users: Table,
	sessions: Table,
}

impl UsersBuilder {
	pub async fn new(db: &Database) -> Self {
		let this = Self {
			users: Table::new("users"),
			sessions: Table::new("user_sessions"),
		};

		let migrations = db.migrations();
		let mut conn = db.get().await.unwrap();

		for (name, sql) in MIGRATIONS {
			migrations
				.add(&mut conn, name, sql)
				.await
				.expect("failed to run migration");
		}

		this
	}

	fn with_connection<'a>(&'a self, conn: Connection<'a>) -> Users<'a> {
		Users {
			users: self.users.with_conn(conn),
			sessions: self.sessions.with_conn(conn),
		}
	}
}

impl UsersBuilderTrait for UsersBuilder {
	fn with_conn<'a>(&'a self, conn: Conn<'a>) -> data::UsersWithConn<'a> {
		Box::new(self.with_connection(conn.pg()))
	}
}

#[derive(Debug, FromRow, ToRow)]
struct UserRow {
	id: UniqueId,
	auth_type: i16,
	oauth_id: Option<String>,
	name: String,
	email: String,
	password: Option<String>,
	created_on: DateTime,
}

impl From<UserRow> for User {
	fn from(row: UserRow) -> Self {
		Self {
			id: row.id,
			name: row.name,
			email: row.email,
			auth: match row.auth_type {
				0 => Auth::Password(row.password.unwrap()),
				1 => Auth::OAuth {
					platform: OAuthPlatform::Github,
					oauth_id: row.oauth_id.unwrap(),
				},
				i => unreachable!("invalid auth type {i}"),
			},
			created_on: row.created_on,
		}
	}
}

#[derive(Debug, FromRow, ToRow)]
struct SessionRow {
	token: Token,
	timeout: Timeout,
	user_id: UniqueId,
	created_on: DateTime,
	oauth_token: Option<String>,
}

impl From<Session> for SessionRow {
	fn from(session: Session) -> Self {
		Self {
			token: session.token,
			timeout: session.timeout,
			user_id: session.user_id,
			created_on: session.created_on,
			oauth_token: session.oauth_token,
		}
	}
}

impl From<SessionRow> for Session {
	fn from(row: SessionRow) -> Self {
		Self {
			token: row.token,
			timeout: row.timeout,
			user_id: row.user_id,
			created_on: row.created_on,
			oauth_token: row.oauth_token,
		}
	}
}

pub struct Users<'a> {
	users: TableWithConn<'a>,
	sessions: TableWithConn<'a>,
}

#[async_trait::async_trait]
impl UsersTrait for Users<'_> {
	async fn by_id(&self, id: &UniqueId) -> Result<Option<User>> {
		self.users
			.select_opt::<UserRow>(filter!(id))
			.await
			.map(|r| r.map(Into::into))
	}

	async fn by_email(&self, email: &str) -> Result<Option<User>> {
		self.users
			.select_opt::<UserRow>(filter!(&email))
			.await
			.map(|r| r.map(Into::into))
	}

	async fn create(&self, user: &User) -> Result<()> {
		let row = match &user.auth {
			Auth::Password(password) => UserRow {
				id: user.id,
				auth_type: 0,
				oauth_id: None,
				name: user.name.clone(),
				email: user.email.clone(),
				password: Some(password.clone()),
				created_on: user.created_on,
			},
			Auth::OAuth {
				platform: OAuthPlatform::Github,
				oauth_id: _,
			} => todo!(),
		};

		self.users.insert(&row).await
	}

	async fn new_session(
		&self,
		user_id: &UniqueId,
		token: Option<String>,
	) -> Result<Session> {
		let session = Session::new(*user_id, token);
		let row = SessionRow::from(session);

		self.sessions.insert(&row).await?;

		Ok(row.into())
	}

	async fn session_by_token(&self, token: &Token) -> Result<Option<Session>> {
		self.sessions
			.select_opt::<SessionRow>(filter!(token))
			.await
			.map(|r| r.map(Into::into))
	}
}
