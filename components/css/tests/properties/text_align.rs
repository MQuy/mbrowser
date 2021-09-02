use dyn_fmt::AsStrFormatExt;
use setup::{assert_css, parse};

#[macro_use]
#[path = "../setup/mod.rs"]
mod setup;

const TEMPLATE: &str = r#"
.name {{
	text-align: {};
}}"#;

#[test]
pub fn keyword() {
	for input in [
		"start",
		"end",
		"left",
		"right",
		"center",
		"justify",
		"match-parent",
		"justify-all",
	]
	.iter()
	{
		let css = &TEMPLATE.format(&[&input]);
		let (stylesheet, _) = parse(css);
		assert_css(&stylesheet, css);
	}
}
