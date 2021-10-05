use dyn_fmt::AsStrFormatExt;
use setup::{assert_css, parse};

#[path = "../values/length.rs"]
mod length;
#[macro_use]
#[path = "../setup/mod.rs"]
mod setup;

const SHORTHAND: &str = r#"
.name {{
	padding: {};
}}"#;

const LONGHAND: &str = r#"
.name {{
	padding-top: {};
	padding-right: {};
	padding-bottom: {};
	padding-left: {};
}}"#;

#[test]
pub fn single_value() {
	for input in ["0.5px", "5%"].iter() {
		let cinput = &SHORTHAND.format(&[&input]);
		let coutput = &LONGHAND.format(&[&input, input, input, input]);
		let (stylesheet, _) = parse(cinput);
		assert_css(&stylesheet, coutput);
	}
}

#[test]
pub fn two_values() {
	for (input, output) in [("1px 5%", ("1px", "5%"))].iter() {
		let cinput = &SHORTHAND.format(&[&input]);
		let coutput = &LONGHAND.format(&[&output.0, output.1, output.0, output.1]);
		let (stylesheet, _) = parse(cinput);
		assert_css(&stylesheet, coutput);
	}
}

#[test]
pub fn three_values() {
	for (input, output) in [("1px 5% 10px", ("1px", "5%", "10px"))].iter() {
		let cinput = &SHORTHAND.format(&[&input]);
		let coutput = &LONGHAND.format(&[&output.0, output.1, output.2, output.1]);
		let (stylesheet, _) = parse(cinput);
		assert_css(&stylesheet, coutput);
	}
}

#[test]
pub fn forth_values() {
	for (input, output) in [("1px 5% 0.25px 5%", ("1px", "5%", "0.25px", "5%"))].iter() {
		let cinput = &SHORTHAND.format(&[&input]);
		let coutput = &LONGHAND.format(&[&output.0, output.1, output.2, output.3]);
		let (stylesheet, _) = parse(cinput);
		assert_css(&stylesheet, coutput);
	}
}
