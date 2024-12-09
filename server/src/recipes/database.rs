use chuchi_postgres::{
	db::Conn,
	table::{table::TableWithConn, Table},
	time::DateTime,
	Connection, Database, FromRow, ToRow, UniqueId,
};

use super::data::{self, RecipesBuilderTrait, RecipesTrait};

const MIGRATIONS: &[(&str, &str)] = migration_files!("create-recipes");

impl RecipesBuilder {
	pub async fn new(db: &Database) -> Self {
		let this = Self {
			recipes: Table::new("recipes"),
			chapters: Table::new("recipe_chapters"),
			ingredients: Table::new("recipe_ingredients"),
			blocks: Table::new("recipe_blocks"),
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

	fn with_connection<'a>(&'a self, conn: Connection<'a>) -> Recipes<'a> {
		Recipes {
			recipes: self.recipes.with_conn(conn),
			chapters: self.chapters.with_conn(conn),
			ingredients: self.ingredients.with_conn(conn),
			blocks: self.blocks.with_conn(conn),
		}
	}
}

impl RecipesBuilderTrait for RecipesBuilder {
	fn with_conn<'a>(&'a self, conn: Conn<'a>) -> data::RecipesWithConn<'a> {
		Box::new(self.with_connection(conn.pg()))
	}
}

#[derive(Debug, FromRow, ToRow)]
struct RecipeRow {
	id: UniqueId,
	creator: UniqueId,
	title: String,
	created_on: DateTime,
	updated_on: DateTime,
}

#[derive(Debug, FromRow, ToRow)]
struct ChapterRow {
	id: UniqueId,
	recipe: UniqueId,
	order: i32,
	title: String,
}

#[derive(Debug, FromRow, ToRow)]
struct IngredientRow {
	id: UniqueId,
	chapter: UniqueId,
	order: i32,
	title: String,
	amount: f64,
	unit: String,
}

#[derive(Debug, FromRow, ToRow)]
struct BlockRow {
	id: UniqueId,
	chapter: UniqueId,
	order: i32,
	text: Option<String>,
}

#[derive(Debug, Clone)]
pub struct RecipesBuilder {
	recipes: Table,
	chapters: Table,
	ingredients: Table,
	blocks: Table,
}

pub struct Recipes<'a> {
	recipes: TableWithConn<'a>,
	chapters: TableWithConn<'a>,
	ingredients: TableWithConn<'a>,
	blocks: TableWithConn<'a>,
}

#[async_trait::async_trait]
impl RecipesTrait for Recipes<'_> {}
