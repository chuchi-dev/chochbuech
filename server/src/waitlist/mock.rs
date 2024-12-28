use std::sync::Arc;

use chuchi_postgres::{db::Conn, Result};

use super::data::{
	self, WaitlistBuilderTrait, WaitlistTrait, WaitlistWithConn,
};

pub struct WaitlistBuilder {
	inner: Arc<Waitlist>,
}

impl WaitlistBuilder {
	pub fn new() -> Self {
		Self {
			inner: Arc::new(Waitlist::new()),
		}
	}
}

impl WaitlistBuilderTrait for WaitlistBuilder {
	fn with_conn<'a>(&'a self, _conn: Conn<'a>) -> WaitlistWithConn<'a> {
		Box::new(self.inner.clone())
	}
}

pub struct Waitlist {}

impl Waitlist {
	pub fn new() -> Self {
		Self {}
	}
}

#[async_trait::async_trait]
impl WaitlistTrait for Arc<Waitlist> {
	async fn by_email(&self, _email: &str) -> Result<Option<data::Waiter>> {
		Ok(None)
	}

	async fn add(&self, _email: &str) -> Result<()> {
		Ok(())
	}
}
