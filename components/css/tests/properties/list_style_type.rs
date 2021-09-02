use content::counter_style_data;
use dyn_fmt::AsStrFormatExt;
use setup::{assert_css, assert_property, parse};

#[path = "../values/content.rs"]
mod content;
#[macro_use]
#[path = "../setup/mod.rs"]
mod setup;

const TEMPLATE: &str = r#"
.name {{
	list-style-type: {};
}}"#;

#[test]
pub fn keyword() {
	for value in ["none", "\"hello world\""].iter() {
		let css = &TEMPLATE.format(&[&value]);
		let (stylesheet, _) = parse(css);
		assert_css(&stylesheet, css);
	}
}

test_property!(counter_style, counter_style_data);
