use dyn_fmt::AsStrFormatExt;
use layout::line_style_data;
use setup::{assert_css, parse};

#[path = "../values/layout.rs"]
mod layout;
#[macro_use]
#[path = "../setup/mod.rs"]
mod setup;

const SHORTHAND: &str = r#"
.name {{
	border-style: {};
}}"#;

const LONGHAND: &str = r#"
.name {{
	border-top-style: {};
	border-right-style: {};
	border-bottom-style: {};
	border-left-style: {};
}}"#;

#[test]
pub fn single_value() {
	for (input, output) in line_style_data().iter() {
		let cinput = &SHORTHAND.format(&[&input]);
		let coutput = &LONGHAND.format(&[&output, output, output, output]);
		let (stylesheet, _) = parse(cinput);
		assert_css(&stylesheet, coutput);
	}
}

#[test]
pub fn two_values() {
	for (input, output) in [
		("none hidden", ("none", "hidden")),
		("dotted dashed", ("dotted", "dashed")),
	]
	.iter()
	{
		let cinput = &SHORTHAND.format(&[&input]);
		let coutput = &LONGHAND.format(&[&output.0, output.1, output.0, output.1]);
		let (stylesheet, _) = parse(cinput);
		assert_css(&stylesheet, coutput);
	}
}

#[test]
pub fn three_values() {
	for (input, output) in [("solid double groove", ("solid", "double", "groove"))].iter() {
		let cinput = &SHORTHAND.format(&[&input]);
		let coutput = &LONGHAND.format(&[&output.0, output.1, output.2, output.1]);
		let (stylesheet, _) = parse(cinput);
		assert_css(&stylesheet, coutput);
	}
}

#[test]
pub fn forth_values() {
	for (input, output) in [("none ridge inset outset", ("none", "ridge", "inset", "outset"))].iter() {
		let cinput = &SHORTHAND.format(&[&input]);
		let coutput = &LONGHAND.format(&[&output.0, output.1, output.2, output.3]);
		let (stylesheet, _) = parse(cinput);
		assert_css(&stylesheet, coutput);
	}
}
