use dyn_fmt::AsStrFormatExt;
use setup::{assert_css, parse};

#[path = "../values/color.rs"]
mod color;
#[path = "../setup/mod.rs"]
mod setup;

const TEMPLATE: &str = r#"
.name {{
	box-shadow: {};
}}"#;

#[test]
pub fn keyword() {
	for value in ["none"].iter() {
		let css = &TEMPLATE.format(&[&value]);
		let (stylesheet, _) = parse(css);
		assert_css(&stylesheet, css);
	}
}

#[test]
pub fn shadows() {
	for (input, output) in [
		("currentColor 10px 2.5px", "currentcolor 10px 2.5px 0px 0px"),
		(
			"transparent 0.5px 11px 2px inset",
			"transparent 0.5px 11px 2px 0px inset",
		),
		("transparent 0.5px 11px 2px 100px", "transparent 0.5px 11px 2px 100px"),
		(
			"transparent 0.5px 11px 0px 0px, currentColor 2px 100px 0px 0px",
			"transparent 0.5px 11px 0px 0px, currentcolor 2px 100px 0px 0px",
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
