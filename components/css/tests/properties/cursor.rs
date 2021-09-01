use dyn_fmt::AsStrFormatExt;
use setup::{assert_css, parse};

#[path = "../setup/mod.rs"]
mod setup;

const TEMPLATE: &str = r#"
.name {{
	cursor: {};
}}"#;

#[test]
pub fn keyword() {
	for value in [
		"auto",
		"default",
		"none",
		"context-menu",
		"help",
		"pointer",
		"progress",
		"wait",
		"cell",
		"crosshair",
		"text",
		"vertical-text",
		"alias",
		"copy",
		"move",
		"no-drop",
		"not-allowed",
		"grab",
		"grabbing",
		"e-resize",
		"n-resize",
		"ne-resize",
		"nw-resize",
		"s-resize",
		"se-resize",
		"sw-resize",
		"w-resize",
		"ew-resize",
		"ns-resize",
		"nesw-resize",
		"nwse-resize",
		"col-resize",
		"row-resize",
		"all-scroll",
		"zoom-in",
		"zoom-out",
	]
	.iter()
	{
		let css = &TEMPLATE.format(&[&value]);
		let (stylesheet, _) = parse(css);
		assert_css(&stylesheet, css);
	}
}

#[test]
pub fn images_keyword() {
	for value in [
		"url(\"http://www.example.com\"), auto",
		"url(\"http://www.example.com\") 0 -5, auto",
		"url(\"http://www.example.com\"), url(\"http://www.example.com\") 5 15, auto",
	]
	.iter()
	{
		let css = &TEMPLATE.format(&[&value]);
		let (stylesheet, _) = parse(css);
		assert_css(&stylesheet, css);
	}
}
