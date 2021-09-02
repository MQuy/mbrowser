use dyn_fmt::AsStrFormatExt;
use setup::{assert_css, parse};

#[macro_use]
#[path = "../setup/mod.rs"]
mod setup;

const TEMPLATE: &str = r#"
.name {{
	font-weight: {};
}}"#;

#[test]
pub fn keyword() {
	for value in ["normal", "bold", "bolder", "lighter"].iter() {
		let css = &TEMPLATE.format(&[&value]);
		let (stylesheet, _) = parse(css);
		assert_css(&stylesheet, css);
	}
}

#[test]
pub fn absolute() {
	for value in [
		"100", "200", "300", "400", "500", "600", "700", "800", "900", "1000",
	]
	.iter()
	{
		let css = &TEMPLATE.format(&[&value]);
		let (stylesheet, _) = parse(css);
		assert_css(&stylesheet, css);
	}
}
