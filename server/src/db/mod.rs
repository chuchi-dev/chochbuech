use chuchi::extractor::Extractor;
use chuchi::extractor_extract;
use chuchi::extractor_prepare;
use chuchi::extractor_validate;
use chuchi::Resource;
use chuchi_postgres::connection as pg_conn;
use chuchi_postgres::database::DatabaseError;
pub use chuchi_postgres::Database as PgDatabase;
pub use chuchi_postgres::Error;

#[derive(Debug, Clone, Resource)]
pub struct Database {
	pg: Option<PgDatabase>,
}

impl Database {
	pub fn new_memory() -> Self {
		Self { pg: None }
	}

	pub async fn get(&self) -> Result<ConnectionOwned, DatabaseError> {
		match &self.pg {
			Some(pg) => Ok(ConnectionOwned {
				pg: Some(pg.get().await?),
			}),
			None => Ok(ConnectionOwned { pg: None }),
		}
	}
}

impl From<PgDatabase> for Database {
	fn from(pg: PgDatabase) -> Self {
		Self { pg: Some(pg) }
	}
}

#[derive(Debug)]
pub struct ConnectionOwned {
	pg: Option<pg_conn::ConnectionOwned>,
}

impl ConnectionOwned {
	pub fn connection(&self) -> Connection {
		Connection {
			pg: self.pg.as_ref().map(|pg| pg.connection()),
		}
	}

	pub async fn transaction(&mut self) -> Result<Transaction, Error> {
		match &mut self.pg {
			Some(pg) => Ok(Transaction {
				pg: Some(pg.transaction().await?),
			}),
			None => Ok(Transaction { pg: None }),
		}
	}
}

impl<'a, R> Extractor<'a, R> for ConnectionOwned {
	type Error = DatabaseError;
	type Prepared = Self;

	extractor_validate!(|validate| {
		assert!(
			validate.resources.exists::<Database>(),
			"Db resource not found"
		);
	});

	extractor_prepare!(|prepare| {
		let db = prepare.resources.get::<Database>().unwrap();
		db.get().await
	});

	extractor_extract!(|extract| { Ok(extract.prepared) });
}

#[derive(Debug)]
pub struct Connection<'a> {
	pg: Option<pg_conn::Connection<'a>>,
}

#[derive(Debug)]
pub struct Transaction<'a> {
	pg: Option<pg_conn::Transaction<'a>>,
}
