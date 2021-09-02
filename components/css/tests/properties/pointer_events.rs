use dyn_fmt::AsStrFormatExt;
use setup::{assert_css, parse};

#[macro_use]
#[path = "../setup/mod.rs"]
mod setup;

const TEMPLATE: &str = r#"
.name {{
	pointer-events: {};
}}"#;

#[test]
pub fn keyword() {
	for (input, output) in [
		("auto", "auto"),
		("bounding-box", "bounding-box"),
		("visiblePainted", "visiblepainted"),
		("visibleFill", "visiblefill"),
		("visibleStroke", "visiblestroke"),
		("visible", "visible"),
		("painted", "painted"),
		("fill", "fill"),
		("stroke", "stroke"),
		("all", "all"),
		("none", "none"),
	]
	.iter()
	{
		let cin = &TEMPLATE.format(&[&input]);
		let cout = &TEMPLATE.format(&[&output]);
		let (stylesheet, _) = parse(cin);
		assert_css(&stylesheet, cout);
	}
}
