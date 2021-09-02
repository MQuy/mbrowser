use dyn_fmt::AsStrFormatExt;
use setup::{assert_css, parse};

#[macro_use]
#[path = "../setup/mod.rs"]
mod setup;

const TEMPLATE: &str = r#"
.name {{
	translate: {};
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
pub fn coordinate() {
	for (input, output) in [
		("10px", "10px 0px 0px"),
		("2.5% 5.75px", "2.5% 5.75px 0px"),
		("1px 0% 100px", "1px 0% 100px"),
	]
	.iter()
	{
		let cin = &TEMPLATE.format(&[&input]);
		let cout = &TEMPLATE.format(&[&output]);
		let (stylesheet, _) = parse(cin);
		assert_css(&stylesheet, cout);
	}
}
