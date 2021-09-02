use dyn_fmt::AsStrFormatExt;
use setup::{assert_css, parse};

#[macro_use]
#[path = "../setup/mod.rs"]
mod setup;

const TEMPLATE: &str = r#"
.name {{
	transition-property: {};
}}"#;

#[test]
pub fn keyword() {
	for input in ["none", "all", "something"].iter() {
		let css = &TEMPLATE.format(&[&input]);
		let (stylesheet, _) = parse(css);
		assert_css(&stylesheet, css);
	}
}

#[test]
pub fn keywords() {
	for input in ["all, bla", "utopia, lala, eww"].iter() {
		let css = &TEMPLATE.format(&[&input]);
		let (stylesheet, _) = parse(css);
		assert_css(&stylesheet, css);
	}
}
