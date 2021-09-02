use dyn_fmt::AsStrFormatExt;
use setup::{assert_css, parse};

#[path = "../setup/mod.rs"]
mod setup;

const TEMPLATE: &str = r#"
.name {{
	direction: {};
}}"#;

#[test]
pub fn keyword() {
	for value in ["ltr", "rtl"].iter() {
		let css = &TEMPLATE.format(&[&value]);
		let (stylesheet, _) = parse(css);
		assert_css(&stylesheet, css);
	}
}
