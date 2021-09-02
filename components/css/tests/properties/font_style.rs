use dyn_fmt::AsStrFormatExt;
use setup::{assert_css, parse};

#[macro_use]
#[path = "../setup/mod.rs"]
mod setup;

const TEMPLATE: &str = r#"
.name {{
	font-style: {};
}}"#;

#[test]
pub fn keyword() {
	for value in ["normal", "italic"].iter() {
		let css = &TEMPLATE.format(&[&value]);
		let (stylesheet, _) = parse(css);
		assert_css(&stylesheet, css);
	}
}

#[test]
pub fn oblique() {
	for (input, output) in [
		("oblique", "oblique 14deg"),
		("oblique 2.5deg", "oblique 2.5deg"),
	]
	.iter()
	{
		let cin = &TEMPLATE.format(&[&input]);
		let cout = &TEMPLATE.format(&[&output]);
		let (stylesheet, _) = parse(cin);
		assert_css(&stylesheet, cout);
	}
}
