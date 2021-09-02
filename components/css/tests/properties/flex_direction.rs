use dyn_fmt::AsStrFormatExt;
use setup::{assert_css, parse};

#[macro_use]
#[path = "../setup/mod.rs"]
mod setup;

const TEMPLATE: &str = r#"
.name {{
	flex-direction: {};
}}"#;

#[test]
pub fn keyword() {
	for value in ["row", "row-reverse", "column", "column-reverse"].iter() {
		let css = &TEMPLATE.format(&[&value]);
		let (stylesheet, _) = parse(css);
		assert_css(&stylesheet, css);
	}
}
