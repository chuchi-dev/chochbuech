//! The cms of chochbuech.com: its content model. The website is the crelte
//! project in `../svelte`. Members log in there (role `member`, no
//! studio) and write their own recipes (zipp-members); favorites, ratings
//! and the planner are mocked in the frontend for now.

mod content;
#[cfg(test)]
mod tests;

use zipp::Cms;

fn main() {
	Cms::new(content::schema(), zipp::migrations!()).run();
}
