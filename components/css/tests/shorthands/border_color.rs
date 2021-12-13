use dyn_fmt::AsStrFormatExt;
use setup::{assert_css, parse};

#[macro_use]
#[path = "../setup/mod.rs"]
mod setup;

const SHORTHAND: &str = r#"
.name {{
	border-color: {};
}}"#;

const LONGHAND: &str = r#"
.name {{
	border-top-color: {};
	border-right-color: {};
	border-bottom-color: {};
	border-left-color: {};
}}"#;

#[test]
pub fn single_value() {
	for (input, output) in [
		("red", "rgb(255 0 0 / 1)"),
		("#f015ca", "rgb(240 21 202 / 1)"),
		("rgb(240,30,50,.7)", "rgb(240 30 50 / 0.7)"),
		("transparent", "transparent"),
	]
	.iter()
	{
		let cinput = &SHORTHAND.format(&[&input]);
		let coutput = &LONGHAND.format(&[&output, output, output, output]);
		let (stylesheet, _) = parse(cinput);
		assert_css(&stylesheet, coutput);
	}
}

#[test]
pub fn two_values() {
	for (input, output) in [
		("red #f015ca", ("rgb(255 0 0 / 1)", "rgb(240 21 202 / 1)")),
		("currentColor transparent", ("currentcolor", "transparent")),
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
	for (input, output) in [(
		"red #f015ca transparent",
		("rgb(255 0 0 / 1)", "rgb(240 21 202 / 1)", "transparent"),
	)]
	.iter()
	{
		let cinput = &SHORTHAND.format(&[&input]);
		let coutput = &LONGHAND.format(&[&output.0, output.1, output.2, output.1]);
		let (stylesheet, _) = parse(cinput);
		assert_css(&stylesheet, coutput);
	}
}

#[test]
pub fn forth_values() {
	for (input, output) in [(
		"red #f015ca currentColor transparent",
		("rgb(255 0 0 / 1)", "rgb(240 21 202 / 1)", "currentcolor", "transparent"),
	)]
	.iter()
	{
		let cinput = &SHORTHAND.format(&[&input]);
		let coutput = &LONGHAND.format(&[&output.0, output.1, output.2, output.3]);
		let (stylesheet, _) = parse(cinput);
		assert_css(&stylesheet, coutput);
	}
}
