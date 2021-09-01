use common::vector::permutate;
use css::str::join_strings;
use dyn_fmt::AsStrFormatExt;
use length::length_data;
use setup::{assert_css, parse};

#[path = "../values/length.rs"]
mod length;
#[macro_use]
#[path = "../setup/mod.rs"]
mod setup;

const TEMPLATE: &str = r#"
.name {{
	border-spacing: {};
}}"#;

#[test]
pub fn length() {
	let mut values = Vec::with_capacity(1);
	for (input, output) in length_data().iter() {
		values.push((input.to_string(), std::format!("{} {}", output, output)))
	}
	for (input, output) in [("10px 5px", "10px 5px"), ("0.25px 1px", "0.25px 1px")].iter() {
		values.push((input.to_string(), output.to_string()));
	}

	for (input, output) in values.iter() {
		let cin = &TEMPLATE.format(&[&input]);
		let cout = &TEMPLATE.format(&[&output]);
		let (stylesheet, _) = parse(cin);
		assert_css(&stylesheet, cout);
	}
}

#[test]
pub fn keyword() {
	for value in ["inherit"].iter() {
		let css = &TEMPLATE.format(&[&value]);
		let (stylesheet, _) = parse(css);
		assert_css(&stylesheet, css);
	}
}
