use chuchi::api::{Method, Request};
use chuchi_crypto::token::Token;
use email_address::EmailAddress;
use serde::{Deserialize, Serialize};

use crate::error::Error;

use super::data::{OAuthPlatform, ShortSession, ShortUser};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginReq {
	pub email: EmailAddress,
	pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Authenticated {
	pub session: ShortSession,
	pub user: ShortUser,
}

impl Request for LoginReq {
	type Response = Authenticated;
	type Error = Error;

	const PATH: &'static str = "/api/users/login";
	const METHOD: Method = Method::POST;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StartOAuthReq {
	pub platform: OAuthPlatform,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StartOAuth {
	pub authorize_url: String,
}

impl Request for StartOAuthReq {
	type Response = StartOAuth;
	type Error = Error;

	const PATH: &'static str = "/api/users/startoauth";
	const METHOD: Method = Method::POST;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EndOAuthReq {
	pub code: String,
	pub state: String,
}

impl Request for EndOAuthReq {
	type Response = Authenticated;
	type Error = Error;

	// api is automatically added by nginx
	const PATH: &'static str = "/api/users/endoauth";
	const METHOD: Method = Method::POST;
}

// login via the session token
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenAuthReq;

impl Request for TokenAuthReq {
	type Response = Authenticated;
	type Error = Error;

	// api is automatically added by nginx
	const PATH: &'static str = "/api/users/tokenauth";
	const METHOD: Method = Method::POST;
	const HEADERS: &'static [&'static str] = &["session-token"];
}

// login via the session token
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogoutReq;

impl Request for LogoutReq {
	type Response = ();
	type Error = Error;

	// api is automatically added by nginx
	const PATH: &'static str = "/api/users/logout";
	const METHOD: Method = Method::POST;
	const HEADERS: &'static [&'static str] = &["session-token"];
}

// // login via the session token
// #[derive(Debug, Clone, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct DeleteReq;

// impl Request for DeleteReq {
// 	type Response = ();
// 	type Error = Error;

// 	// api is automatically added by nginx
// 	const PATH: &'static str = "/api/users/delete";
// 	const METHOD: Method = Method::POST;
// 	const HEADERS: &'static [&'static str] = &["session-token"];
// }
