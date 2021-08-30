use percentage::ratio_data;
use setup::{assert_css, assert_property, parse};

#[path = "../values/percentage.rs"]
mod percentage;
#[path = "../setup/mod.rs"]
mod setup;

const TEMPLATE: &str = r#"
.name {{
	aspect-ratio: {};
}}
    "#;

#[test]
pub fn only_auto() {
	let css = r#"
.name {
	aspect-ratio: auto;
}
    "#;
	let (stylesheet, _) = parse(css);
	assert_css(&stylesheet, css);
}

#[test]
pub fn only_ratio() {
	for (input, output) in ratio_data().iter() {
		assert_property(TEMPLATE, input, output);
	}
}

#[test]
pub fn auto_and_ratio() {
	let css = r#"
.name {
	aspect-ratio: auto 1 / 0;
}
    "#;
	let (stylesheet, _) = parse(css);
	assert_css(&stylesheet, css);
}

#[test]
pub fn ratio_and_auto() {
	let css = r#"
.name {
	aspect-ratio: 5.5 / 2.5 auto;
}
    "#;
	let output = r#"
.name {
	aspect-ratio: auto 5.5 / 2.5;
}
    "#;
	let (stylesheet, _) = parse(css);
	assert_css(&stylesheet, output);
}
