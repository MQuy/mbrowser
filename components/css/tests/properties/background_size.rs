use dyn_fmt::AsStrFormatExt;
use length::length_percentage_or_auto_data;
use setup::{assert_css, parse};

#[path = "../values/length.rs"]
mod length;
#[macro_use]
#[path = "../setup/mod.rs"]
mod setup;

const TEMPLATE: &str = r#"
.name {{
	background-size: {};
}}"#;

#[test]
pub fn length_percentage_or_auto() {
	for (input, output) in length_percentage_or_auto_data().iter() {
		let cin = &TEMPLATE.format(&[&input]);
		let cout = &TEMPLATE.format(&[std::format!("{} auto", output)]);
		let (stylesheet, _) = parse(cin);
		assert_css(&stylesheet, cout);
	}
}

#[test]
pub fn length_percentage_or_autos() {
	for value in ["10px auto", "0.25px 5%"].iter() {
		let css = &TEMPLATE.format(&[&value]);
		let (stylesheet, _) = parse(css);
		assert_css(&stylesheet, css);
	}
}

#[test]
pub fn keyword() {
	for value in ["cover", "contain"].iter() {
		let css = &TEMPLATE.format(&[&value]);
		let (stylesheet, _) = parse(css);
		assert_css(&stylesheet, css);
	}
}

#[test]
pub fn bg_sizes() {
	for (input, output) in [
		("auto, cover", "auto auto, cover"),
		("10px, 5.5%", "10px auto, 5.5% auto"),
		("10px 100%, contain", "10px 100%, contain"),
	]
	.iter()
	{
		let cin = &TEMPLATE.format(&[&input]);
		let cout = &TEMPLATE.format(&[&output]);
		let (stylesheet, _) = parse(cin);
		assert_css(&stylesheet, cout);
	}
}
