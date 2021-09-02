use dyn_fmt::AsStrFormatExt;
use setup::{assert_css, parse};

#[macro_use]
#[path = "../setup/mod.rs"]
mod setup;

const TEMPLATE: &str = r#"
.name {{
	font-family: {};
}}"#;

#[test]
pub fn family_name() {
	for value in ["Helvetica", "\"Times New Roman\"", "Helvetica, Verdana"].iter() {
		let css = &TEMPLATE.format(&[&value]);
		let (stylesheet, _) = parse(css);
		assert_css(&stylesheet, css);
	}
}

#[test]
pub fn generic_family() {
	for value in [
		"serif",
		"sans-serif",
		"cursive",
		"fantasy",
		"monospace",
		"system-ui",
		"emoji",
		"math",
		"fangsong",
		"ui-serif",
		"ui-sans-serif",
		"ui-monospace",
		"ui-rounded",
	]
	.iter()
	{
		let css = &TEMPLATE.format(&[&value]);
		let (stylesheet, _) = parse(css);
		assert_css(&stylesheet, css);
	}
}

#[test]
pub fn names() {
	for value in ["Helvetica, Verdana, sans-serif"].iter() {
		let css = &TEMPLATE.format(&[&value]);
		let (stylesheet, _) = parse(css);
		assert_css(&stylesheet, css);
	}
}
