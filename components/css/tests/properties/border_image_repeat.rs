use common::vector::permutate;
use dyn_fmt::AsStrFormatExt;
use setup::{assert_css, parse};

#[path = "../setup/mod.rs"]
mod setup;

const TEMPLATE: &str = r#"
.name {{
	border-image-repeat: {};
}}"#;

#[test]
pub fn keyword() {
	let mut values = Vec::with_capacity(1);
	let keywords = ["stretch", "repeat", "round", "space"];
	for value in keywords.iter() {
		values.push((value.to_string(), std::format!("{} {}", value, value)));
	}
	for (first, second) in permutate(keywords.iter(), keywords.iter()).iter() {
		let value = std::format!("{} {}", first, second);
		values.push((value.clone(), value.clone()));
	}
	for (input, output) in values.iter() {
		let cin = &TEMPLATE.format(&[&input]);
		let cout = &TEMPLATE.format(&[&output]);
		let (stylesheet, _) = parse(cin);
		assert_css(&stylesheet, cout);
	}
}
