use chuchi::{api, Chuchi};
use chuchi_postgres::db::ConnOwned;

use crate::error::{Error, Result};
use crate::users::api::{Authenticated, TokenAuthReq};
use crate::users::data::{Auth, User};
use crate::users::{api::LoginReq, Users};

use super::utils::AuthedUser;

// or #[api("/api/login")]
// async fn login(req: LoginReq, auth_token: Header<String>)
#[api(LoginReq)]
async fn login(
	req: LoginReq,
	users: &Users,
	db: ConnOwned,
) -> Result<Authenticated> {
	let users = users.with_conn(db.conn());

	let user = users.by_email(req.email.as_str()).await?;

	let Some(user) = user else {
		return Err(Error::LoginIncorrect);
	};

	match &user {
		User {
			auth: Auth::Password(pw),
			..
		} if bcrypt::verify(req.password.as_str(), pw).unwrap_or(false) => {
			let session = users.new_session(&user.id, None).await?;

			Ok(Authenticated {
				user: user.into(),
				session: session.to_short(),
			})
		}
		_ => Err(Error::LoginIncorrect),
	}
}

#[api(TokenAuthReq)]
async fn token_auth(user: AuthedUser) -> Result<Authenticated> {
	Ok(Authenticated {
		user: user.user.into(),
		session: user.session.to_short(),
	})
}

pub fn routes(server: &mut Chuchi) {
	server.add_route(login);
	server.add_route(token_auth);
}
