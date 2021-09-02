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
	vertical-align: {};
}}"#;

#[test]
pub fn keyword() {
	for input in [
		"baseline",
		"sub",
		"super",
		"top",
		"text-top",
		"middle",
		"bottom",
		"text-bottom",
	]
	.iter()
	{
		let css = &TEMPLATE.format(&[&input]);
		let (stylesheet, _) = parse(css);
		assert_css(&stylesheet, css);
	}
}

test_property!(length_percentage, length_percentage_data);
