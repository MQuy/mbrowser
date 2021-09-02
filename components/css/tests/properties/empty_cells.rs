use dyn_fmt::AsStrFormatExt;
use setup::{assert_css, parse};

#[path = "../setup/mod.rs"]
mod setup;

const TEMPLATE: &str = r#"
.name {{
	empty-cells: {};
}}"#;

#[test]
pub fn keyword() {
	for value in ["show", "hide"].iter() {
		let css = &TEMPLATE.format(&[&value]);
		let (stylesheet, _) = parse(css);
		assert_css(&stylesheet, css);
	}
}
