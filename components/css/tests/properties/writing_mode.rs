use dyn_fmt::AsStrFormatExt;
use setup::{assert_css, parse};

#[macro_use]
#[path = "../setup/mod.rs"]
mod setup;

const TEMPLATE: &str = r#"
.name {{
	writing-mode: {};
}}"#;

#[test]
pub fn keyword() {
	for input in [
		"horizontal-tb",
		"vertical-rl",
		"vertical-lr",
		"sideways-rl",
		"sideways-lr",
	]
	.iter()
	{
		let css = &TEMPLATE.format(&[&input]);
		let (stylesheet, _) = parse(css);
		assert_css(&stylesheet, css);
	}
}
