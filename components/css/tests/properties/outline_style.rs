use dyn_fmt::AsStrFormatExt;
use layout::line_style_data;
use setup::{assert_css, assert_property, parse};

#[path = "../values/layout.rs"]
mod layout;
#[macro_use]
#[path = "../setup/mod.rs"]
mod setup;

const TEMPLATE: &str = r#"
.name {{
	outline-style: {};
}}"#;

#[test]
pub fn invert() {
	for value in ["auto"].iter() {
		let css = &TEMPLATE.format(&[&value]);
		let (stylesheet, _) = parse(css);
		assert_css(&stylesheet, css);
	}
}

fn line_style_without_hidden_data() -> Vec<(String, String)> {
	let data = line_style_data();
	data.iter()
		.filter(|(input, _output)| input != "hidden")
		.map(|(input, output)| (input.to_string(), output.to_string()))
		.collect::<Vec<(String, String)>>()
}

test_property!(line_style, line_style_without_hidden_data);
