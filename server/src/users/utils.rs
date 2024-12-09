use super::{
	data::{Session, Token, User, UsersWithConn},
	Users,
};
use crate::error::Error;

use chuchi::{
	extractor::Extractor, extractor_extract, extractor_prepare,
	extractor_validate, header::RequestHeader,
};
use chuchi_postgres::db::Db;

pub struct AuthedUser {
	pub session: Session,
	pub user: User,
}

impl<'a, R> Extractor<'a, R> for AuthedUser {
	type Error = Error;
	type Prepared = Self;

	extractor_validate!(|validate| {
		assert!(
			validate.resources.exists::<Users>(),
			"Users resource not found"
		);
		assert!(validate.resources.exists::<Db>(), "Db resource not found");
	});

	extractor_prepare!(|prepare| {
		let db = prepare.resources.get::<Db>().unwrap();
		let conn = db.get().await?;
		let users = prepare.resources.get::<Users>().unwrap();
		let users = users.with_conn(conn.conn());

		let session = session_from_req(prepare.header, &users).await?;

		let user = authenticated_user(&session, &users).await?;

		Ok(Self { session, user })
	});

	extractor_extract!(|extract| { Ok(extract.prepared) });
}

pub async fn session_from_req(
	header: &RequestHeader,
	users: &UsersWithConn<'_>,
) -> Result<Session, Error> {
	let token: Token = header
		.value("session-token")
		.ok_or(Error::MissingSessionToken)?
		.parse()
		.map_err(|_| Error::MissingSessionToken)?;

	users
		.session_by_token(&token)
		.await
		.map_err(|e| Error::Internal(e.to_string()))?
		.ok_or(Error::InvalidSessionToken)
}

pub async fn authenticated_user(
	session: &Session,
	users: &UsersWithConn<'_>,
) -> Result<User, Error> {
	let user = users
		.by_id(&session.user_id)
		.await
		.map_err(|e| Error::Internal(e.to_string()))?
		.ok_or(Error::NotFound)?;

	Ok(user)
}
