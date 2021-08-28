use common::vector::permutate;
use setup::{assert_css, parse};

#[path = "../setup/mod.rs"]
mod setup;

#[test]
pub fn keyword() {
	for prefix in ["auto", "normal", "stretch"].iter() {
		let css = &std::format!(
			r#"
.name {{
	align-self: {};
}}
    "#,
			prefix
		);
		let (stylesheet, _) = parse(css);
		assert_css(&stylesheet, css);
	}
}

#[test]
pub fn baseline() {
	for prefix in ["first", "last"].iter() {
		let css = &std::format!(
			r#"
.name {{
	align-self: {} baseline;
}}
    "#,
			prefix
		);
		let (stylesheet, _) = parse(css);
		assert_css(&stylesheet, css);
	}
}

#[test]
pub fn overflow_and_content() {
	for (prefix, content) in permutate(
		["unsafe", "safe"].iter(),
		["center", "start", "end", "flex-start", "flex-end"].iter(),
	) {
		let css = &std::format!(
			r#"
.name {{
	align-self: {} {};
}}
    "#,
			prefix,
			content
		);
		let (stylesheet, _) = parse(css);
		assert_css(&stylesheet, css);
	}
}

#[test]
pub fn only_content() {
	for prefix in ["center", "start", "end", "flex-start", "flex-end"].iter() {
		let css = &std::format!(
			r#"
.name {{
	align-self: {};
}}
    "#,
			prefix
		);
		let (stylesheet, _) = parse(css);
		assert_css(&stylesheet, css);
	}
}
