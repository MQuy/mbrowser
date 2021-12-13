use dyn_fmt::AsStrFormatExt;
use setup::{assert_css, parse};

#[macro_use]
#[path = "../setup/mod.rs"]
mod setup;

const SHORTHAND: &str = r#"
.name {{
	border-width: {};
}}"#;

const LONGHAND: &str = r#"
.name {{
	border-top-width: {};
	border-right-width: {};
	border-bottom-width: {};
	border-left-width: {};
}}"#;

#[test]
pub fn single_value() {
	for input in ["1.5px", "thin", "medium", "thick"].iter() {
		let cinput = &SHORTHAND.format(&[&input]);
		let coutput = &LONGHAND.format(&[&input, input, input, input]);
		let (stylesheet, _) = parse(cinput);
		assert_css(&stylesheet, coutput);
	}
}

#[test]
pub fn two_values() {
	for (input, output) in [("10px 5px", ("10px", "5px")), ("thin thick", ("thin", "thick"))].iter() {
		let cinput = &SHORTHAND.format(&[&input]);
		let coutput = &LONGHAND.format(&[&output.0, output.1, output.0, output.1]);
		let (stylesheet, _) = parse(cinput);
		assert_css(&stylesheet, coutput);
	}
}

#[test]
pub fn three_values() {
	for (input, output) in [("1px 5px medium", ("1px", "5px", "medium"))].iter() {
		let cinput = &SHORTHAND.format(&[&input]);
		let coutput = &LONGHAND.format(&[&output.0, output.1, output.2, output.1]);
		let (stylesheet, _) = parse(cinput);
		assert_css(&stylesheet, coutput);
	}
}

#[test]
pub fn forth_values() {
	for (input, output) in [("1px thin thick 50px", ("1px", "thin", "thick", "50px"))].iter() {
		let cinput = &SHORTHAND.format(&[&input]);
		let coutput = &LONGHAND.format(&[&output.0, output.1, output.2, output.3]);
		let (stylesheet, _) = parse(cinput);
		assert_css(&stylesheet, coutput);
	}
}
