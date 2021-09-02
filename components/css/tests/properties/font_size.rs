use dyn_fmt::AsStrFormatExt;
use length::length_percentage_data;
use setup::{assert_css, assert_property, parse};

#[path = "../values/length.rs"]
mod length;
#[macro_use]
#[path = "../setup/mod.rs"]
mod setup;

const TEMPLATE: &str = r#"
.name {{
	font-size: {};
}}"#;

#[test]
pub fn absolute_size() {
	for value in [
		"xx-small", "x-small", "small", "medium", "large", "x-large", "xx-large",
	]
	.iter()
	{
		let css = &TEMPLATE.format(&[&value]);
		let (stylesheet, _) = parse(css);
		assert_css(&stylesheet, css);
	}
}

#[test]
pub fn relative_size() {
	for value in ["larger", "smaller"].iter() {
		let css = &TEMPLATE.format(&[&value]);
		let (stylesheet, _) = parse(css);
		assert_css(&stylesheet, css);
	}
}

test_property!(length_percentage, length_percentage_data);
