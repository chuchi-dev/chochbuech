use chuchi::api;

use crate::error::{Error, Result};
use crate::users::api::Authenticated;
use crate::users::data::{Auth, User};
use crate::users::{api::LoginReq, Users};

// or #[api("/api/login")]
// async fn login(req: LoginReq, auth_token: Header<String>)
#[api(LoginReq)]
async fn login(req: LoginReq, users: &Users) -> Result<Authenticated> {
	let user = users.by_email(req.email.as_str()).await?;

	let Some(user) = user else {
		return Err(Error::LoginIncorrect);
	};

	match &user {
		User {
			auth: Auth::Password(pw),
			..
		} if pw == &req.password => {
			let session = users.new_session(&user.id, None).await?;

			Ok(Authenticated {
				user: user.into(),
				session: session.to_short(),
			})
		}
		_ => Err(Error::LoginIncorrect),
	}
}
