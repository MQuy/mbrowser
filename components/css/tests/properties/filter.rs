use dyn_fmt::AsStrFormatExt;
use filter::filter_list_data;
use setup::{assert_css, assert_property, parse};

#[path = "../values/filter.rs"]
mod filter;
#[macro_use]
#[path = "../setup/mod.rs"]
mod setup;

const TEMPLATE: &str = r#"
.name {{
	filter: {};
}}"#;

#[test]
pub fn keyword() {
	for value in ["none"].iter() {
		let css = &TEMPLATE.format(&[&value]);
		let (stylesheet, _) = parse(css);
		assert_css(&stylesheet, css);
	}
}

test_property!(value_list, filter_list_data);

#[test]
pub fn values() {
	for (input, output) in [(
		"url(\"http://www.example.com\") blur() contrast()",
		"url(\"http://www.example.com\") blur(0px) contrast(1)",
	)]
	.iter()
	{
		let cin = &TEMPLATE.format(&[&input]);
		let cout = &TEMPLATE.format(&[&output]);
		let (stylesheet, _) = parse(cin);
		assert_css(&stylesheet, cout);
	}
}
