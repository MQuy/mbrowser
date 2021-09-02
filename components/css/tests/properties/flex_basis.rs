use dyn_fmt::AsStrFormatExt;
use length::size_data;
use setup::{assert_css, assert_property, parse};

#[path = "../values/length.rs"]
mod length;
#[macro_use]
#[path = "../setup/mod.rs"]
mod setup;

const TEMPLATE: &str = r#"
.name {{
	flex-basis: {};
}}"#;

#[test]
pub fn keyword() {
	for value in ["content"].iter() {
		let css = &TEMPLATE.format(&[&value]);
		let (stylesheet, _) = parse(css);
		assert_css(&stylesheet, css);
	}
}

test_property!(size, size_data);
