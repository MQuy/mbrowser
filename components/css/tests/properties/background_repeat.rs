use common::vector::permutate;
use css::str::join_strings;
use dyn_fmt::AsStrFormatExt;
use setup::{assert_css, parse};

#[path = "../values/length.rs"]
mod length;
#[macro_use]
#[path = "../setup/mod.rs"]
mod setup;

const TEMPLATE: &str = r#"
.name {{
	background-repeat: {};
}}"#;

#[test]
pub fn keyword() {
	let mut values = Vec::with_capacity(1);
	let keywords = ["repeat", "space", "round", "no-repeat"];
	for (horizontal, vertical) in permutate(keywords.iter(), keywords.iter()).iter() {
		let input = join_strings(vec![horizontal, vertical], " ");
		values.push((input.to_string(), input.to_string()));
	}

	for (input, output) in [
		("repeat-x", "repeat no-repeat"),
		("repeat-y", "no-repeat repeat"),
		("repeat", "repeat repeat"),
		("space", "space space"),
		("round", "round round"),
		("no-repeat", "no-repeat no-repeat"),
	]
	.iter()
	{
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
pub fn keywords() {
	let cin = &TEMPLATE.format(&[&"repeat-x, space, repeat repeat"]);
	let cout = &TEMPLATE.format(&[&"repeat no-repeat, space space, repeat repeat"]);
	let (stylesheet, _) = parse(cin);
	assert_css(&stylesheet, cout);
}
