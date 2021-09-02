use dyn_fmt::AsStrFormatExt;
use setup::{assert_css, parse};

#[macro_use]
#[path = "../setup/mod.rs"]
mod setup;

const TEMPLATE: &str = r#"
.name {{
	position: {};
}}"#;

#[test]
pub fn keyword() {
	for input in ["static", "absolute", "relative", "fixed", "sticky"].iter() {
		let css = &TEMPLATE.format(&[&input]);
		let (stylesheet, _) = parse(css);
		assert_css(&stylesheet, css);
	}
}
