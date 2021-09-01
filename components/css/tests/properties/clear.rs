use dyn_fmt::AsStrFormatExt;
use setup::{assert_css, parse};

#[path = "../values/color.rs"]
mod color;
#[path = "../setup/mod.rs"]
mod setup;

const TEMPLATE: &str = r#"
.name {{
	clear: {};
}}"#;

#[test]
pub fn keyword() {
	for value in [
		"none",
		"left",
		"right",
		"both",
		"inline-start",
		"inline-end",
	]
	.iter()
	{
		let css = &TEMPLATE.format(&[&value]);
		let (stylesheet, _) = parse(css);
		assert_css(&stylesheet, css);
	}
}
