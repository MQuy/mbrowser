use dyn_fmt::AsStrFormatExt;
use setup::{assert_css, parse};

#[path = "../values/color.rs"]
mod color;
#[path = "../setup/mod.rs"]
mod setup;

const TEMPLATE: &str = r#"
.name {{
	box-sizing: {};
}}"#;

#[test]
pub fn keyword() {
	for value in ["content-box", "border-box"].iter() {
		let css = &TEMPLATE.format(&[&value]);
		let (stylesheet, _) = parse(css);
		assert_css(&stylesheet, css);
	}
}
