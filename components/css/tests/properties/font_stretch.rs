use dyn_fmt::AsStrFormatExt;
use percentage::percentage_data;
use setup::{assert_css, assert_property, parse};

#[path = "../values/percentage.rs"]
mod percentage;
#[macro_use]
#[path = "../setup/mod.rs"]
mod setup;

const TEMPLATE: &str = r#"
.name {{
	font-stretch: {};
}}"#;

#[test]
pub fn keyword() {
	for value in [
		"normal",
		"ultra-condensed",
		"extra-condensed",
		"condensed",
		"semi-condensed",
		"semi-expanded",
		"expanded",
		"extra-expanded",
		"ultra-expanded",
	]
	.iter()
	{
		let css = &TEMPLATE.format(&[&value]);
		let (stylesheet, _) = parse(css);
		assert_css(&stylesheet, css);
	}
}

test_property!(percentage, percentage_data);
