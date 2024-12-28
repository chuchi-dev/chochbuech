use chuchi_postgres::{
	db::Conn,
	filter,
	table::{table::TableWithConn, Table},
	time::DateTime,
	Connection, Database, FromRow, Result, ToRow,
};

use super::data::{self, WaitlistBuilderTrait, WaitlistTrait};

const MIGRATIONS: &[(&str, &str)] = migration_files!("waitlist-01-create");

#[derive(Debug, Clone)]
pub struct WaitlistBuilder {
	table: Table,
}

impl WaitlistBuilder {
	pub async fn new(db: &Database) -> Self {
		let this = Self {
			table: Table::new("waitlist"),
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

	fn with_connection<'a>(&'a self, conn: Connection<'a>) -> Waitlist<'a> {
		Waitlist {
			table: self.table.with_conn(conn),
		}
	}
}

impl WaitlistBuilderTrait for WaitlistBuilder {
	fn with_conn<'a>(&'a self, conn: Conn<'a>) -> data::WaitlistWithConn<'a> {
		Box::new(self.with_connection(conn.pg()))
	}
}

#[derive(Debug, Clone, FromRow, ToRow)]
pub struct Waiter {
	pub email: String,
	pub created_on: DateTime,
}

impl From<Waiter> for data::Waiter {
	fn from(w: Waiter) -> Self {
		Self {
			email: w.email,
			created_on: w.created_on,
		}
	}
}

pub struct Waitlist<'a> {
	table: TableWithConn<'a>,
}

#[async_trait::async_trait]
impl WaitlistTrait for Waitlist<'_> {
	async fn by_email(&self, email: &str) -> Result<Option<data::Waiter>> {
		self.table
			.select_opt::<Waiter>(filter!(&email))
			.await
			.map(|opt| opt.map(Into::into))
	}

	async fn add(&self, email: &str) -> Result<()> {
		let waiter = Waiter {
			email: email.into(),
			created_on: DateTime::now(),
		};

		self.table.insert(&waiter).await.map(|_| ())
	}
}
