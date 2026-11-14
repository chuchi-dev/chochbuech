//! The cms of chochbuech.com: its content model. The website is the crelte
//! project in `../svelte`. Members log in there (role `member`, no
//! studio) and write their own recipes (zipp-members); favorites, ratings
//! and the planner are mocked in the frontend for now. An unpublished
//! recipe is a private one of its author; `images` gives the website its
//! photos, `import` reads a recipe of another site into the editor with
//! the recipe `assistant`.

mod assistant;
mod content;
mod images;
mod import;
#[cfg(test)]
mod tests;

use zipp::Cms;

fn main() {
	Cms::new(content::schema(), zipp::migrations!())
		.routes(import::routes(import::Import::from_env()))
		.routes(images::routes())
		.run();
}
