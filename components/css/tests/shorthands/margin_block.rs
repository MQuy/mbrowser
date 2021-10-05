use dyn_fmt::AsStrFormatExt;
use setup::{assert_css, parse};

#[path = "../values/length.rs"]
mod length;
#[macro_use]
#[path = "../setup/mod.rs"]
mod setup;

const SHORTHAND: &str = r#"
.name {{
	margin-block: {};
}}"#;

const LONGHAND: &str = r#"
.name {{
	margin-block-start: {};
	margin-block-end: {};
}}"#;

#[test]
pub fn single_value() {
	for input in ["-1.5px", "auto", "5%"].iter() {
		let cinput = &SHORTHAND.format(&[&input]);
		let coutput = &LONGHAND.format(&[&input, input, input, input]);
		let (stylesheet, _) = parse(cinput);
		assert_css(&stylesheet, coutput);
	}
}

#[test]
pub fn two_values() {
	for (input, output) in [("10px 5%", ("10px", "5%")), ("auto auto", ("auto", "auto"))].iter() {
		let cinput = &SHORTHAND.format(&[&input]);
		let coutput = &LONGHAND.format(&[&output.0, output.1, output.0, output.1]);
		let (stylesheet, _) = parse(cinput);
		assert_css(&stylesheet, coutput);
	}
}
