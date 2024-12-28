use chuchi::{api, Chuchi};
use chuchi_postgres::db::ConnOwned;

use crate::error::Result;

use super::api::AddWaitlistReq;
use super::data::Waitlist;

#[api(AddWaitlistReq)]
async fn add_waitlist(
	req: AddWaitlistReq,
	waitlist: &Waitlist,
	mut db: ConnOwned,
) -> Result<()> {
	let trans = db.trans().await?;
	{
		let waitlist = waitlist.with_conn(trans.conn());

		if let Some(_) = waitlist.by_email(req.email.as_str()).await? {
			return Ok(());
		}

		waitlist.add(req.email.as_str()).await?;
	}

	trans.commit().await?;

	Ok(())
}

pub fn routes(server: &mut Chuchi) {
	server.add_route(add_waitlist);
}
