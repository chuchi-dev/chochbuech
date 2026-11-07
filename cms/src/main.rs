//! The cms of chochbuech.com: its content model. The website is the crelte
//! project in `../svelte`. What visitors do themselves (signing in,
//! favorites, ratings, writing recipes) is mocked in the frontend for now,
//! zipp only has the studio's users.

mod content;
#[cfg(test)]
mod tests;

use zipp::Cms;

fn main() {
	Cms::new(content::schema(), zipp::migrations!()).run();
}
