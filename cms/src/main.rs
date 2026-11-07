//! The cms of chochbuech.com: its content model. The website is the crelte
//! project in `../svelte`. Members log in there (role `member`, no
//! studio); what they do (favorites, ratings, writing recipes) is mocked in
//! the frontend for now.

mod content;
#[cfg(test)]
mod tests;

use zipp::Cms;

fn main() {
	Cms::new(content::schema(), zipp::migrations!()).run();
}
