use chuchi::impl_res_extractor;
use chuchi_postgres::{db::Conn, time::DateTime, UniqueId};

#[derive(Debug, Clone)]
pub struct Recipe {
	pub id: UniqueId,
	pub creator: UniqueId,
	pub title: String,
	// category
	pub instructions: Vec<Chapter>,
	pub created_on: DateTime,
	pub updated_on: DateTime,
}

#[derive(Debug, Clone)]
pub struct Chapter {
	pub title: String,
	pub ingredients: Vec<Ingredient>,
	pub content: Blocks,
}

#[derive(Debug, Clone)]
pub struct Ingredient {
	pub title: String,
	pub amount: Amount,
}

#[derive(Debug, Clone)]
pub struct Amount {
	pub value: f64,
	pub unit: Unit,
}

#[derive(Debug, Clone)]
pub enum Unit {
	Gram,
	Kilogram,
	Milliliter,
	Deziliter,
	Liter,
}

#[derive(Debug, Clone)]
pub struct Blocks(pub Vec<Block>);

#[derive(Debug, Clone)]
pub enum Block {
	Text(String),
	// Image(String),
}

pub type Recipes = Box<dyn RecipesBuilderTrait + Send + Sync>;
pub type RecipesWithConn<'a> = Box<dyn RecipesTrait + Send + Sync + 'a>;

impl_res_extractor!(Recipes);

pub trait RecipesBuilderTrait {
	fn with_conn<'a>(&'a self, conn: Conn<'a>) -> RecipesWithConn<'a>;
}

#[async_trait::async_trait]
pub trait RecipesTrait {}
