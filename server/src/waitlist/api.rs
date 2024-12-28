use chuchi::api::{Method, Request};
use email_address::EmailAddress;
use serde::{Deserialize, Serialize};

use crate::error::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddWaitlistReq {
	pub email: EmailAddress,
}

impl Request for AddWaitlistReq {
	type Response = ();
	type Error = Error;

	const PATH: &'static str = "/api/waitlist/add";
	const METHOD: Method = Method::POST;
}
