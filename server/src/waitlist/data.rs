use chuchi::impl_res_extractor;
use chuchi_postgres::{db::Conn, time::DateTime, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Waiter {
	pub email: String,
	pub created_on: DateTime,
}

pub type Waitlist = Box<dyn WaitlistBuilderTrait + Send + Sync>;
pub type WaitlistWithConn<'a> = Box<dyn WaitlistTrait + Send + Sync + 'a>;

impl_res_extractor!(Waitlist);

pub trait WaitlistBuilderTrait {
	fn with_conn<'a>(&'a self, conn: Conn<'a>) -> WaitlistWithConn<'a>;
}

#[async_trait::async_trait]
pub trait WaitlistTrait {
	async fn by_email(&self, email: &str) -> Result<Option<Waiter>>;

	async fn add(&self, email: &str) -> Result<()>;
}
