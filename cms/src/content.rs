//! The content model of chochbuech.com: sites, sections, entry types and
//! blocks.

use zipp::{
	Access, Column, Field, Format, LinkKind, Role, Schema, Section, Transform,
};
use zipp_members::Members;
use zipp_seo::{Seo, SeoDefaults, seo_field};

/// The units of an ingredient's amount, `piece` shows the number alone.
const UNITS: [(&str, &str); 10] = [
	("piece", "Pieces"),
	("g", "Grams"),
	("kg", "Kilograms"),
	("ml", "Milliliters"),
	("cl", "Centiliters"),
	("dl", "Deciliters"),
	("l", "Liters"),
	("tsp", "Teaspoons"),
	("tbsp", "Tablespoons"),
	("pinch", "Pinch"),
];

pub fn schema() -> Schema {
	Schema::new()
		.site("en", "Chuchi Recipes", "en")
		.timezone("Europe/Zurich")
		.section(
			Section::structure("pages", ["home", "page", "recipes"])
				.uri("{parent.uri}/{slug}")
				.uri_of("home", ""),
		)
		.section(Section::channel("recipes", ["recipe"]).uri("recipes/{slug}"))
		// the header's menu and the filters are in the order of the tree
		.section(Section::structure("categories", ["category"]).max_level(1))
		// the people behind the recipes, their public page; a member's is
		// the cook they are the author of
		.section(Section::channel("cooks", ["cook"]).uri("cooks/{slug}"))
		.section(Section::single("header", "header"))
		.section(Section::single("footer", "footer"))
		.nav("Content", ["pages", "recipes", "categories", "cooks"])
		.nav("Settings", ["header", "footer", "seoSettings"])
		.nav_icon("Content", "folder-open")
		.nav_icon("Settings", "sliders-horizontal")
		// the visitors with an account, they log in on the website only
		// (no studio) and write their own recipes there, live at once
		.role(
			Role::new("member", "Member")
				// only their own recipes: View of the section would let
				// them read the others' unpublished ones on the website
				.own("recipes", Access::Delete)
				// what their recipes link to
				.section("categories", Access::View)
				.section("cooks", Access::View)
				// the images of their recipes
				.assets(Access::Edit)
				.no_studio(),
		)
		// the website's editor: /members/* of the crelte server
		.plugin(
			Members::new()
				.section("recipes")
				.images(8 * 1024 * 1024, "Members/{user}"),
		)
		.plugin(
			Seo::new()
				.entry_type(
					"recipe",
					SeoDefaults::new()
						.title_pattern("{title} | {siteName}")
						.image_from("image"),
				)
				.entry_type("cook", SeoDefaults::new().description_from("bio")),
		)
		// the cards of the lists are 3:4
		.transform(
			Transform::new("card")
				.width(480)
				.height(640)
				.crop()
				.format(Format::Webp),
		)
		.transform(
			Transform::new("thumb")
				.width(200)
				.height(264)
				.crop()
				.format(Format::Webp),
		)
		.transform(
			Transform::new("hero")
				.width(2200)
				.height(1400)
				.crop()
				.format(Format::Webp),
		)
		.transform(Transform::new("wide").width(1300).format(Format::Webp))
		.transform(
			Transform::new("collage")
				.width(720)
				.height(720)
				.crop()
				.format(Format::Webp),
		)
		.entry_tabs(
			"home",
			"Home",
			[
				(
					"Hero",
					vec![
						Field::rich_text("heroTitle")
							.required()
							.label("Title")
							.instructions("*Italic* words are set in italics."),
						Field::images("heroImages").max(3).label("Images").instructions(
							"Up to three, placed around the title: top left, bottom left, right.",
						),
					],
				),
				(
					"Content",
					vec![Field::blocks(
						"content",
						["recipeRow", "feature", "columns"],
					)],
				),
				("SEO", vec![seo_field("seo")]),
			],
		)
		.entry_tabs(
			"page",
			"Page",
			[
				(
					"Content",
					vec![
						Field::rich_text("text")
							.headings([2, 3])
							.lists()
							.links(),
					],
				),
				("SEO", vec![seo_field("seo")]),
			],
		)
		// the list of all recipes with the search and the filters
		.entry_tabs(
			"recipes",
			"Recipes",
			[(
				"Content",
				vec![Field::note(
					"listNote",
					"Lists every recipe with a search and the categories as filters; \
					the header's categories link here with `?category=<slug>`.",
				)],
			),
			("SEO", vec![seo_field("seo")])],
		)
		.entry_tabs(
			"recipe",
			"Recipe",
			[
				(
					"Recipe",
					vec![
						Field::relation("category", ["category"])
							.max(1)
							.required()
							.width(50),
						Field::relation("cook", ["cook"])
							.max(1)
							.label("By")
							.instructions("Empty: Anonymous.")
							.width(50),
						Field::images("image")
							.max(1)
							.required()
							.folder("Recipes/{slug}"),
						Field::number("serves")
							.range(1.0, 100.0)
							.decimals(0)
							.unit("people")
							.default(4)
							.width(25),
						Field::number("totalTime")
							.range(0.0, 10000.0)
							.decimals(0)
							.unit("min")
							.label("Time in total")
							.width(25),
						Field::number("activeTime")
							.range(0.0, 10000.0)
							.decimals(0)
							.unit("min")
							.label("Time actively")
							.width(25),
						Field::number("rating")
							.range(0.0, 5.0)
							.decimals(1)
							.indexed()
							.instructions(
								"The average of the visitors' ratings; set by hand until they can rate.",
							)
							.width(25),
					],
				),
				(
					"Instructions",
					vec![
						Field::blocks("chapters", ["chapter"]).instructions(
							"A chapter has its ingredients and its steps: every paragraph is a step, an image belongs to the step above it.",
						),
					],
				),
			],
		)
		.entry(
			"category",
			"Category",
			[Field::text("singular")
				.required()
				.instructions("One of them: *Main Course by …* on the cards.")],
		)
		.entry(
			"cook",
			"Cook",
			[
				Field::number("memberSince")
					.range(2000.0, 2100.0)
					.decimals(0)
					.label("Member since")
					.width(25),
				Field::text("bio").multiline(),
			],
		)
		.entry(
			"header",
			"Header",
			[Field::relation("categories", ["category"])
				.label("Categories in the menu")],
		)
		.entry(
			"footer",
			"Footer",
			[
				Field::text("copyright").instructions("After the ©, e.g. Chuchi Recipes, 2024."),
				Field::navigation("links", [LinkKind::Entry, LinkKind::Url])
					.sections(["pages"]),
			],
		)
		// one chapter of a recipe: Chutney with figs, Finishing, ...
		.block(
			"chapter",
			"Chapter",
			[
				Field::text("title").instructions(
					"Empty in the first chapter: Ingredients and Instructions.",
				),
				Field::table(
					"ingredients",
					[
						Column::number("amount").decimals(2),
						Column::dropdown("unit", UNITS),
						Column::text("name").label("Ingredient"),
					],
				),
				Field::rich_text("steps").links().embed(["stepImage"]),
			],
		)
		.block(
			"stepImage",
			"Image",
			[Field::images("image").max(1).required()],
		)
		// the blocks of the home page
		.block(
			"recipeRow",
			"Recipes",
			[
				Field::text("marquee").instructions(
					"Runs through behind the recipes, e.g. *trending recipes*.",
				),
				Field::relation("recipes", ["recipe"]).required(),
				Field::boolean("slider")
					.instructions("Scrolls sideways with arrows, else one row."),
			],
		)
		.block(
			"feature",
			"Feature",
			[
				Field::rich_text("title").required(),
				Field::rich_text("text").links(),
				Field::link("link", [LinkKind::Entry, LinkKind::Url])
					.label("Button"),
				Field::dropdown("look", [("plain", "Plain"), ("blue", "Blue")])
					.default("plain")
					.width(50),
				Field::boolean("doodles")
					.instructions(
						"The hand drawn corrections over the title (made for *There's too much sugar in this cake.*).",
					)
					.width(50),
			],
		)
		.block(
			"columns",
			"Text in two columns",
			[
				Field::rich_text("left").links(),
				Field::rich_text("right").links(),
			],
		)
		.icon("home", "house")
		.icon("page", "file-text")
		.icon("recipes", "list")
		.icon("recipe", "chef-hat")
		.icon("category", "tag")
		.icon("cook", "user-round")
		.icon("chapter", "list-ordered")
		.icon("stepImage", "image")
		.icon("recipeRow", "gallery-horizontal")
		.icon("feature", "megaphone")
		.icon("columns", "columns-2")
		.block_title("chapter", "title")
		.block_title("recipeRow", "marquee")
		.block_title("feature", "title")
		.entry_card("cook")
}
