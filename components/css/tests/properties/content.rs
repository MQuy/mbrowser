use content::{counter_data, leader_data, target_data};
use dyn_fmt::AsStrFormatExt;
use setup::{assert_css, parse};

#[path = "../values/content.rs"]
mod content;
#[path = "../setup/mod.rs"]
mod setup;

const TEMPLATE: &str = r#"
.name {{
	content: {};
}}"#;

#[test]
pub fn keyword() {
	for value in ["normal", "none"].iter() {
		let css = &TEMPLATE.format(&[&value]);
		let (stylesheet, _) = parse(css);
		assert_css(&stylesheet, css);
	}
}

#[test]
pub fn content() {
	let mut values = Vec::with_capacity(1);
	for value in [
		"element(#id)",
		"\"hello world\"",
		"contents",
		"open-quote",
		"close-quote",
		"no-open-quote",
		"no-close-quote",
	]
	.iter()
	{
		values.push((value.to_string(), value.to_string()));
	}
	for (input, output) in counter_data().iter() {
		values.push((input.to_string(), output.to_string()));
	}
	for (input, output) in target_data().iter() {
		values.push((input.to_string(), output.to_string()));
	}
	for (input, output) in leader_data().iter() {
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
pub fn contents() {
	for value in [
		"contents open-quote / \"hello\"",
		"contents close-quote / counter(something) \"lalala\"",
	]
	.iter()
	{
		let css = &TEMPLATE.format(&[&value]);
		let (stylesheet, _) = parse(css);
		assert_css(&stylesheet, css);
	}
}
