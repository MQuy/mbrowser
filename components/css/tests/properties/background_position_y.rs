use common::vector::permutate;
use css::str::join_strings;
use dyn_fmt::AsStrFormatExt;
use length::length_percentage_data;
use setup::{assert_css, parse};

#[path = "../values/length.rs"]
mod length;
#[macro_use]
#[path = "../setup/mod.rs"]
mod setup;

const TEMPLATE: &str = r#"
.name {{
	background-position-y: {};
}}"#;

#[test]
pub fn center() {
	let css = &TEMPLATE.format(&[&"center"]);
	let (stylesheet, _) = parse(css);
	assert_css(&stylesheet, css);
}

#[test]
pub fn horizontal_component() {
	let mut length_percentage_data = length_percentage_data();
	length_percentage_data.push(("".to_string(), "".to_string()));

	for (keyword, (length_input, _)) in permutate(
		["top", "bottom", "y-start", "y-end", ""].iter(),
		length_percentage_data.iter(),
	)
	.iter()
	{
		if keyword.len() == 0 && length_input.len() == 0 {
			continue;
		}
		let input = join_strings(vec![keyword, length_input], " ");
		let css = &TEMPLATE.format(&[&input]);
		let (stylesheet, _) = parse(css);
		assert_css(&stylesheet, css);
	}
}

#[test]
pub fn center_horizontal_components() {
	let css = &TEMPLATE.format(&[&"center, top 10px, 10%"]);
	let (stylesheet, _) = parse(css);
	println!("{}", stylesheet.to_string());
	assert_css(&stylesheet, css);
}
