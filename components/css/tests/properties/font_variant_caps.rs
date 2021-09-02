use dyn_fmt::AsStrFormatExt;
use setup::{assert_css, parse};

#[macro_use]
#[path = "../setup/mod.rs"]
mod setup;

const TEMPLATE: &str = r#"
.name {{
	font-variant-caps: {};
}}"#;

#[test]
pub fn keyword() {
	for value in [
		"normal",
		"small-caps",
		"all-small-caps",
		"petite-caps",
		"all-petite-caps",
		"unicase",
		"titling-caps",
	]
	.iter()
	{
		let css = &TEMPLATE.format(&[&value]);
		let (stylesheet, _) = parse(css);
		assert_css(&stylesheet, css);
	}
}
