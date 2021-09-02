use dyn_fmt::AsStrFormatExt;
use setup::{assert_css, parse};

#[macro_use]
#[path = "../setup/mod.rs"]
mod setup;

const TEMPLATE: &str = r#"
.name {{
	text-rendering: {};
}}"#;

#[test]
pub fn keyword() {
	for (input, output) in [
		("auto", "auto"),
		("optimizeSpeed", "optimizespeed"),
		("optimizeLegibility", "optimizelegibility"),
		("geometricPrecision", "geometricprecision"),
	]
	.iter()
	{
		let cin = &TEMPLATE.format(&[&input]);
		let cout = &TEMPLATE.format(&[&output]);
		let (stylesheet, _) = parse(cin);
		assert_css(&stylesheet, cout);
	}
}
