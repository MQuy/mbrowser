use dyn_fmt::AsStrFormatExt;
use setup::{assert_css, parse};

#[macro_use]
#[path = "../setup/mod.rs"]
mod setup;

const TEMPLATE: &str = r#"
.name {{
	unicode-bidi: {};
}}"#;

#[test]
pub fn keyword() {
	for input in [
		"normal",
		"embed",
		"isolate",
		"bidi-override",
		"isolate-override",
		"plantext",
	]
	.iter()
	{
		let css = &TEMPLATE.format(&[&input]);
		let (stylesheet, _) = parse(css);
		assert_css(&stylesheet, css);
	}
}
