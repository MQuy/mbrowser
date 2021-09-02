use common::vector::permutate;
use dyn_fmt::AsStrFormatExt;
use setup::{assert_css, parse};

#[macro_use]
#[path = "../setup/mod.rs"]
mod setup;

const TEMPLATE: &str = r#"
.name {{
	text-overflow: {};
}}"#;

#[test]
pub fn edge() {
	for input in [
		"clip",
		"ellipsis",
		"\"hello\"",
		"fade",
		"fade(10px)",
		"fade(5%)",
	]
	.iter()
	{
		let css = &TEMPLATE.format(&[&input]);
		let (stylesheet, _) = parse(css);
		assert_css(&stylesheet, css);
	}
}

#[test]
pub fn edges() {
	let keywords = [
		"clip",
		"ellipsis",
		"\"hello\"",
		"fade",
		"fade(10px)",
		"fade(5%)",
	];

	for (left, right) in permutate(keywords.iter(), keywords.iter()).iter() {
		let input = std::format!("{} {}", left, right);
		let css = &TEMPLATE.format(&[&input]);
		let (stylesheet, _) = parse(css);
		assert_css(&stylesheet, css);
	}
}
