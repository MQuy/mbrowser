use dyn_fmt::AsStrFormatExt;
use setup::{assert_css, parse};

#[macro_use]
#[path = "../setup/mod.rs"]
mod setup;

const TEMPLATE: &str = r#"
.name {{
	float: {};
}}"#;

#[test]
pub fn keyword() {
	for value in ["left", "right", "none", "inline-start", "inline-end"].iter() {
		let css = &TEMPLATE.format(&[&value]);
		let (stylesheet, _) = parse(css);
		assert_css(&stylesheet, css);
	}
}
