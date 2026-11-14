//! What a page says about its recipe, before the model reads it: the
//! schema.org Recipe of its JSON-LD (most recipe sites have one for
//! search engines), else its main text.

use serde::Serialize;
use serde_json::Value;

/// The text of a page without a Recipe, at most this many characters.
pub const MAX_TEXT: usize = 12_000;
/// A step or an ingredient of the JSON-LD, at most.
const MAX_ITEM: usize = 2_000;
const MAX_ITEMS: usize = 200;

/// A schema.org Recipe, the parts the editor has.
#[derive(Debug, Default, Clone, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Source {
	pub name: String,
	#[serde(skip_serializing_if = "String::is_empty")]
	pub description: String,
	#[serde(skip_serializing_if = "String::is_empty")]
	pub category: String,
	pub serves: Option<u32>,
	/// minutes
	pub total_time: Option<u32>,
	pub prep_time: Option<u32>,
	pub cook_time: Option<u32>,
	pub ingredients: Vec<String>,
	pub sections: Vec<Section>,
}

/// The steps of a HowToSection, the steps without one have no title.
#[derive(Debug, Default, Clone, Serialize, PartialEq)]
pub struct Section {
	#[serde(skip_serializing_if = "String::is_empty")]
	pub title: String,
	pub steps: Vec<String>,
}

/// The first schema.org Recipe of the page's JSON-LD scripts.
pub fn json_ld_recipe(html: &str) -> Option<Source> {
	scripts(html)
		.filter_map(|s| serde_json::from_str::<Value>(s.trim()).ok())
		.find_map(|v| find_recipe(&v, 0).map(source_of))
}

/// The contents of the `<script type="application/ld+json">` tags.
fn scripts(html: &str) -> impl Iterator<Item = &str> {
	// ASCII lower case keeps the byte offsets
	let lower = html.to_ascii_lowercase();
	let mut found = vec![];
	let mut at = 0;
	while let Some(start) = lower[at..].find("<script").map(|i| at + i) {
		let Some(open_end) = lower[start..].find('>').map(|i| start + i + 1)
		else {
			break;
		};
		let Some(end) =
			lower[open_end..].find("</script").map(|i| open_end + i)
		else {
			break;
		};
		if lower[start..open_end].contains("application/ld+json") {
			found.push(&html[open_end..end]);
		}
		at = end;
	}
	found.into_iter()
}

fn is_recipe(v: &Value) -> bool {
	match &v["@type"] {
		Value::String(t) => t == "Recipe" || t.ends_with("/Recipe"),
		Value::Array(ts) => ts.iter().any(|t| t == "Recipe"),
		_ => false,
	}
}

/// The Recipe anywhere in a JSON-LD document: at the top, in an array,
/// in `@graph` or as the `mainEntity` of a page.
fn find_recipe(v: &Value, depth: usize) -> Option<&Value> {
	if depth > 6 {
		return None;
	}
	match v {
		Value::Object(o) => {
			if is_recipe(v) {
				return Some(v);
			}
			o.values().find_map(|v| find_recipe(v, depth + 1))
		}
		Value::Array(a) => a.iter().find_map(|v| find_recipe(v, depth + 1)),
		_ => None,
	}
}

fn source_of(r: &Value) -> Source {
	let ingredients = match r.get("recipeIngredient").or(r.get("ingredients")) {
		Some(Value::Array(a)) => a.iter().filter_map(text_of).collect(),
		Some(v) => text_of(v).into_iter().collect(),
		None => vec![],
	};
	let mut sections = vec![];
	instructions(&r["recipeInstructions"], &mut sections, 0);
	sections.retain(|s: &Section| !s.steps.is_empty());
	let prep_time = minutes(&r["prepTime"]);
	let cook_time = minutes(&r["cookTime"]);
	let total_time =
		minutes(&r["totalTime"]).or(match (prep_time, cook_time) {
			(Some(p), Some(c)) => Some(p + c),
			_ => None,
		});
	Source {
		name: text_of(&r["name"]).unwrap_or_default(),
		description: text_of(&r["description"]).unwrap_or_default(),
		category: match &r["recipeCategory"] {
			Value::Array(a) => {
				a.iter().filter_map(text_of).collect::<Vec<_>>().join(", ")
			}
			v => text_of(v).unwrap_or_default(),
		},
		serves: serves(&r["recipeYield"]),
		total_time,
		prep_time,
		cook_time,
		ingredients: capped(ingredients),
		sections,
	}
}

fn capped(mut items: Vec<String>) -> Vec<String> {
	items.truncate(MAX_ITEMS);
	items
}

/// The steps of `recipeInstructions`: a text, a list of texts, of
/// HowToSteps or of HowToSections with their steps. A step goes into
/// the last section.
fn instructions(v: &Value, sections: &mut Vec<Section>, depth: usize) {
	if depth > 8 {
		return;
	}
	match v {
		Value::String(s) => {
			for line in clean(s).lines() {
				let line =
					line.split_whitespace().collect::<Vec<_>>().join(" ");
				if !line.is_empty() {
					push_step(sections, line);
				}
			}
		}
		Value::Array(a) => {
			for v in a {
				instructions(v, sections, depth + 1);
			}
		}
		Value::Object(_) => {
			let ty = v["@type"].as_str().unwrap_or("");
			let items = &v["itemListElement"];
			if ty == "HowToSection" {
				let title = text_of(&v["name"]).unwrap_or_default();
				sections.push(Section {
					title,
					steps: vec![],
				});
				instructions(items, sections, depth + 1);
				// what follows the section has no title again
				sections.push(Section::default());
			} else if let Some(t) = text_of(&v["text"]).or(text_of(&v["name"]))
			{
				push_step(sections, t);
			} else {
				// an ItemList, a HowToStep of HowToDirections
				instructions(items, sections, depth + 1);
			}
		}
		_ => {}
	}
}

fn push_step(sections: &mut Vec<Section>, step: String) {
	if sections.is_empty() {
		sections.push(Section::default());
	}
	let s = sections.last_mut().unwrap();
	if s.steps.len() < MAX_ITEMS {
		s.steps.push(step);
	}
}
/// A plain text of a JSON-LD value: no tags, no entities, one line.
fn text_of(v: &Value) -> Option<String> {
	let s = match v {
		Value::String(s) => clean(s),
		Value::Number(n) => n.to_string(),
		Value::Object(_) => {
			return text_of(&v["@value"]).or(text_of(&v["name"]));
		}
		_ => return None,
	};
	let s = s.split_whitespace().collect::<Vec<_>>().join(" ");
	(!s.is_empty()).then(|| truncate(&s, MAX_ITEM).to_string())
}

/// The text of a bit of html: the tags out, the entities decoded.
fn clean(s: &str) -> String {
	decode_entities(&strip_tags(s))
}

/// The people of `recipeYield`: `4`, `"4 Portionen"`, `["4", "4
/// servings"]`, `"4-6"` (the first number).
pub fn serves(v: &Value) -> Option<u32> {
	match v {
		Value::Number(n) => n.as_f64().map(|n| n.round() as u32),
		Value::String(s) => {
			let digits: String = s
				.trim_start_matches(|c: char| !c.is_ascii_digit())
				.chars()
				.take_while(|c| c.is_ascii_digit())
				.collect();
			digits.parse().ok()
		}
		Value::Array(a) => a.iter().find_map(serves),
		_ => None,
	}
	.filter(|n| (1..=100).contains(n))
}

fn minutes(v: &Value) -> Option<u32> {
	v.as_str().and_then(duration_minutes)
}

/// The minutes of an ISO 8601 duration: `PT1H30M`, `P0DT20M`, `PT90M`.
pub fn duration_minutes(s: &str) -> Option<u32> {
	let s = s.trim().to_ascii_uppercase();
	let rest = s.strip_prefix('P')?;
	let mut minutes = 0.0;
	let mut num = String::new();
	let mut time = false;
	let mut any = false;
	for c in rest.chars() {
		match c {
			'0'..='9' | '.' | ',' => num.push(if c == ',' { '.' } else { c }),
			'T' => time = true,
			_ => {
				let n: f64 = num.parse().ok()?;
				num.clear();
				any = true;
				minutes += match (c, time) {
					('W', false) => n * 7.0 * 24.0 * 60.0,
					('D', false) => n * 24.0 * 60.0,
					('H', true) => n * 60.0,
					('M', true) => n,
					('S', true) => n / 60.0,
					_ => return None,
				};
			}
		}
	}
	if !any || !num.is_empty() || minutes > 10_000.0 {
		return None;
	}
	Some(minutes.round() as u32)
}

/// The visible text of a page without a Recipe: its title and the text
/// of its `<main>` (or `<article>`, or the body) without the scripts,
/// menus, headers and footers, at most [`MAX_TEXT`] characters.
pub fn main_text(html: &str) -> String {
	let title = element(html, "title")
		.map(|t| decode_entities(&strip_tags(t)))
		.unwrap_or_default();
	let mut body = html.to_string();
	for name in [
		"head", "script", "style", "noscript", "svg", "template", "iframe",
		"nav", "header", "footer", "aside", "form", "button",
	] {
		body = remove_elements(&body, name);
	}
	let main = element(&body, "main")
		.or_else(|| element(&body, "article"))
		.or_else(|| element(&body, "body"))
		.unwrap_or(&body);
	let text = decode_entities(&strip_tags(main));
	let mut out = String::new();
	if !title.trim().is_empty() {
		out.push_str(title.trim());
		out.push_str("\n\n");
	}
	for line in text.lines() {
		let line = line.split_whitespace().collect::<Vec<_>>().join(" ");
		if !line.is_empty() {
			out.push_str(&line);
			out.push('\n');
		}
	}
	truncate(&out, MAX_TEXT).to_string()
}

/// The inner html of the first element `name`.
fn element<'a>(html: &'a str, name: &str) -> Option<&'a str> {
	let lower = html.to_ascii_lowercase();
	let start = find_tag(&lower, name, 0)?;
	let inner = start + lower[start..].find('>')? + 1;
	let end = lower[inner..]
		.find(&format!("</{name}"))
		.map(|i| inner + i)
		.unwrap_or(html.len());
	Some(&html[inner..end])
}

/// The position of the opening tag `<name` (not `<names`) from `at`.
fn find_tag(lower: &str, name: &str, mut at: usize) -> Option<usize> {
	let open = format!("<{name}");
	loop {
		let i = at + lower[at..].find(&open)?;
		let next = lower[i + open.len()..].chars().next();
		if next.is_none_or(|c| c == '>' || c == '/' || c.is_whitespace()) {
			return Some(i);
		}
		at = i + open.len();
	}
}

/// The html without the elements `name` and what is in them.
fn remove_elements(html: &str, name: &str) -> String {
	let lower = html.to_ascii_lowercase();
	let close = format!("</{name}");
	let mut out = String::with_capacity(html.len());
	let mut at = 0;
	while let Some(start) = find_tag(&lower, name, at) {
		out.push_str(&html[at..start]);
		at = match lower[start..].find(&close) {
			Some(i) => {
				let end = start + i;
				lower[end..].find('>').map_or(html.len(), |j| end + j + 1)
			}
			None => html.len(),
		};
	}
	out.push_str(&html[at.min(html.len())..]);
	out
}

/// The text of html: tags out, a line break for the block ones.
fn strip_tags(html: &str) -> String {
	let mut out = String::with_capacity(html.len());
	let mut rest = html;
	while let Some(i) = rest.find('<') {
		out.push_str(&rest[..i]);
		let Some(j) = rest[i..].find('>') else {
			rest = "";
			break;
		};
		let tag = rest[i + 1..i + j]
			.trim_start_matches('/')
			.to_ascii_lowercase();
		let name: String = tag
			.chars()
			.take_while(|c| c.is_ascii_alphanumeric())
			.collect();
		if matches!(
			name.as_str(),
			"p" | "br"
				| "div" | "li"
				| "tr" | "h1"
				| "h2" | "h3"
				| "h4" | "h5"
				| "h6" | "section"
				| "ul" | "ol"
				| "table" | "dd"
				| "dt"
		) {
			out.push('\n');
		} else {
			out.push(' ');
		}
		rest = &rest[i + j + 1..];
	}
	out.push_str(rest);
	out
}

/// `&amp;`, `&#252;`, `&#xFC;` and the usual named entities.
pub fn decode_entities(s: &str) -> String {
	let mut out = String::with_capacity(s.len());
	let mut rest = s;
	while let Some(i) = rest.find('&') {
		out.push_str(&rest[..i]);
		rest = &rest[i..];
		let end = rest[1..].find(';').map(|j| j + 1).filter(|j| *j <= 10);
		let decoded = end.and_then(|end| {
			let e = &rest[1..end];
			let c = if let Some(hex) =
				e.strip_prefix("#x").or(e.strip_prefix("#X"))
			{
				u32::from_str_radix(hex, 16).ok().and_then(char::from_u32)
			} else if let Some(dec) = e.strip_prefix('#') {
				dec.parse().ok().and_then(char::from_u32)
			} else {
				named_entity(e)
			};
			c.map(|c| (c, end + 1))
		});
		match decoded {
			Some((c, len)) => {
				out.push(c);
				rest = &rest[len..];
			}
			None => {
				out.push('&');
				rest = &rest[1..];
			}
		}
	}
	out.push_str(rest);
	out
}

fn named_entity(e: &str) -> Option<char> {
	Some(match e {
		"amp" => '&',
		"lt" => '<',
		"gt" => '>',
		"quot" => '"',
		"apos" => '\'',
		"nbsp" => ' ',
		"auml" => 'ä',
		"ouml" => 'ö',
		"uuml" => 'ü',
		"Auml" => 'Ä',
		"Ouml" => 'Ö',
		"Uuml" => 'Ü',
		"szlig" => 'ß',
		"eacute" => 'é',
		"egrave" => 'è',
		"agrave" => 'à',
		"ccedil" => 'ç',
		"frac12" => '½',
		"frac14" => '¼',
		"frac34" => '¾',
		"deg" => '°',
		"ndash" => '–',
		"mdash" => '—',
		"hellip" => '…',
		"rsquo" => '’',
		"lsquo" => '‘',
		"rdquo" => '”',
		"ldquo" => '“',
		"laquo" => '«',
		"raquo" => '»',
		"times" => '×',
		_ => return None,
	})
}

/// At most `max` bytes, cut at a character.
pub fn truncate(s: &str, max: usize) -> &str {
	if s.len() <= max {
		return s;
	}
	let mut end = max;
	while !s.is_char_boundary(end) {
		end -= 1;
	}
	&s[..end]
}
