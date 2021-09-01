use dyn_fmt::AsStrFormatExt;
use setup::{assert_css, parse};

#[path = "../values/color.rs"]
mod color;
#[path = "../setup/mod.rs"]
mod setup;

const TEMPLATE: &str = r#"
.name {{
	caption-side: {};
}}"#;

#[test]
pub fn keyword() {
	for value in ["top", "bottom"].iter() {
		let css = &TEMPLATE.format(&[&value]);
		let (stylesheet, _) = parse(css);
		assert_css(&stylesheet, css);
	}
}
