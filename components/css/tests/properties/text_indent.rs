use dyn_fmt::AsStrFormatExt;
use setup::{assert_css, parse};

#[macro_use]
#[path = "../setup/mod.rs"]
mod setup;

const TEMPLATE: &str = r#"
.name {{
	text-indent: {};
}}"#;

#[test]
pub fn content() {
	for input in [
		"10px",
		"5.5%",
		"10px hanging",
		"20% each-line",
		"2.5px hanging each-line",
	]
	.iter()
	{
		let css = &TEMPLATE.format(&[&input]);
		let (stylesheet, _) = parse(css);
		assert_css(&stylesheet, css);
	}
}
