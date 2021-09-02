use dyn_fmt::AsStrFormatExt;
use setup::{assert_css, parse};

#[macro_use]
#[path = "../setup/mod.rs"]
mod setup;

const TEMPLATE: &str = r#"
.name {{
	text-transform: {};
}}"#;

#[test]
pub fn keyword() {
	for input in [
		"none",
		"capitalize",
		"uppercase",
		"lowercase",
		"full-width",
		"full-size-kana",
		"capitalize uppercase",
		"full-width full-size-kana",
		"capitalize lowercase full-width",
		"capitalize uppercase full-size-kana",
		"capitalize uppercase lowercase full-width",
		"capitalize uppercase lowercase full-size-kana",
		"capitalize uppercase lowercase full-width full-size-kana",
	]
	.iter()
	{
		let css = &TEMPLATE.format(&[&input]);
		let (stylesheet, _) = parse(css);
		assert_css(&stylesheet, css);
	}
}
