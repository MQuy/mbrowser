use dyn_fmt::AsStrFormatExt;
use setup::{assert_css, parse};

#[macro_use]
#[path = "../setup/mod.rs"]
mod setup;

const TEMPLATE: &str = r#"
.name {{
	text-decoration-line: {};
}}"#;

#[test]
pub fn keyword() {
	for input in [
		"none",
		"underline",
		"overline",
		"line-through",
		"blink",
		"underline overline",
		"overline blink",
		"line-through blink",
		"underline overline line-through",
		"underline overline blink",
		"underline overline line-through blink",
	]
	.iter()
	{
		let css = &TEMPLATE.format(&[&input]);
		let (stylesheet, _) = parse(css);
		assert_css(&stylesheet, css);
	}
}
