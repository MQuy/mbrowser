use animation::keyframe_name_data;
use setup::{assert_css, assert_property, parse};

#[path = "../values/animation.rs"]
mod animation;
#[path = "../setup/mod.rs"]
mod setup;

const TEMPLATE: &str = r#"
.name {{
	animation-name: {};
}}
    "#;

#[test]
pub fn keyword() {
	let css = r#"
.name {
	animation-name: none;
}
    "#;
	let (stylesheet, _) = parse(css);
	assert_css(&stylesheet, css);
}

#[test]
pub fn keyframe_name() {
	for (input, output) in keyframe_name_data().iter() {
		assert_property(TEMPLATE, input, output);
	}
}

#[test]
pub fn mixed_keyword_and_keyframe_name() {
	let css = r#"
.name {
	animation-name: none, momo, "hello";
}
    "#;
	let (stylesheet, _) = parse(css);
	assert_css(&stylesheet, css);
}
