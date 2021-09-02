use dyn_fmt::AsStrFormatExt;
use setup::{assert_css, parse};

#[macro_use]
#[path = "../setup/mod.rs"]
mod setup;

const TEMPLATE: &str = r#"
.name {{
	justify-content: {};
}}"#;

#[test]
pub fn keyword() {
	for value in [
		"flex-start",
		"stretch",
		"flex-end",
		"center",
		"space-between",
		"space-around",
	]
	.iter()
	{
		let css = &TEMPLATE.format(&[&value]);
		let (stylesheet, _) = parse(css);
		assert_css(&stylesheet, css);
	}
}
