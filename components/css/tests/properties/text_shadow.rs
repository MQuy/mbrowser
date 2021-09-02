use dyn_fmt::AsStrFormatExt;
use setup::{assert_css, parse};

#[macro_use]
#[path = "../setup/mod.rs"]
mod setup;

const TEMPLATE: &str = r#"
.name {{
	text-shadow: {};
}}"#;

#[test]
pub fn keyword() {
	for input in ["none"].iter() {
		let css = &TEMPLATE.format(&[&input]);
		let (stylesheet, _) = parse(css);
		assert_css(&stylesheet, css);
	}
}

#[test]
pub fn shadow() {
	for (input, output) in [
		("transparent 10px 2.5px", "transparent 10px 2.5px 0px"),
		("0px 1px 1.25px", "currentcolor 0px 1px 1.25px"),
		("currentColor 0px 1px 0px", "currentcolor 0px 1px 0px"),
		(
			"currentColor 0px 1px, transparent 0.5px .5px",
			"currentcolor 0px 1px 0px, transparent 0.5px 0.5px 0px",
		),
	]
	.iter()
	{
		let cin = &TEMPLATE.format(&[&input]);
		let cout = &TEMPLATE.format(&[&output]);
		let (stylesheet, _) = parse(cin);
		assert_css(&stylesheet, cout);
	}
}
