use common::vector::permutate;
use css::str::join_strings;
use dyn_fmt::AsStrFormatExt;
use setup::{assert_css, parse};

#[path = "../setup/mod.rs"]
mod setup;

const TEMPLATE: &str = r#"
.name {{
	display: {};
}}"#;

#[test]
pub fn outside_inside() {
	for (outside, inside) in permutate(
		["block", "inline", "run-in", ""].iter(),
		["flow", "flow-root", "table", "flex", "grid", "ruby", ""].iter(),
	)
	.iter()
	{
		if outside.len() == 0 && inside.len() == 0 {
			continue;
		}
		let value = join_strings(vec![outside, inside], " ");
		let css = &TEMPLATE.format(&[&value]);
		let (stylesheet, _) = parse(css);
		assert_css(&stylesheet, css);
	}
}

#[test]
pub fn listitem() {
	for (outside, inside) in permutate(
		["block", "inline", "run-in", ""].iter(),
		["flow", "flow-root", ""].iter(),
	)
	.iter()
	{
		let value = join_strings(vec![outside, inside, "list-item"], " ");
		let css = &TEMPLATE.format(&[&value]);
		let (stylesheet, _) = parse(css);
		assert_css(&stylesheet, css);
	}
}

#[test]
pub fn internal() {
	for value in [
		"table-row-group",
		"table-header-group",
		"table-footer-group",
		"table-row",
		"table-cell",
		"table-column-group",
		"table-column",
		"table-caption",
		"ruby-base",
		"ruby-text",
		"ruby-base-container",
		"ruby-text-container",
	]
	.iter()
	{
		let css = &TEMPLATE.format(&[&value]);
		let (stylesheet, _) = parse(css);
		assert_css(&stylesheet, css);
	}
}

#[test]
pub fn box_() {
	for value in ["contents", "none"].iter() {
		let css = &TEMPLATE.format(&[&value]);
		let (stylesheet, _) = parse(css);
		assert_css(&stylesheet, css);
	}
}

#[test]
pub fn legacy() {
	for value in ["inline-block", "inline-table", "inline-flex", "inline-grid"].iter() {
		let css = &TEMPLATE.format(&[&value]);
		let (stylesheet, _) = parse(css);
		assert_css(&stylesheet, css);
	}
}
