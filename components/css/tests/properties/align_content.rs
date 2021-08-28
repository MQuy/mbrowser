use common::vector::permutate;
use setup::{assert_css, parse};

#[path = "../setup/mod.rs"]
mod setup;

#[test]
pub fn normal() {
	let css = r#"
.name {
	align-content: normal;
}
    "#;
	let (stylesheet, _) = parse(css);
	assert_css(&stylesheet, css);
}

#[test]
pub fn only_baseline() {
	let css = r#"
.name {
	align-content: baseline;
}
    "#;
	let output = r#"
.name {
	align-content: first baseline;
}
    "#;
	let (stylesheet, _) = parse(css);
	assert_css(&stylesheet, output);
}

#[test]
pub fn baseline() {
	for prefix in ["first", "last"].iter() {
		let css = &std::format!(
			r#"
.name {{
	align-content: {} baseline;
}}
    "#,
			prefix
		);
		let (stylesheet, _) = parse(css);
		assert_css(&stylesheet, css);
	}
}

#[test]
pub fn distribution() {
	for prefix in ["space-between", "space-around", "space-evenly", "stretch"].iter() {
		let css = &std::format!(
			r#"
.name {{
	align-content: {};
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
	align-content: {} {};
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
	align-content: {};
}}
    "#,
			prefix
		);
		let (stylesheet, _) = parse(css);
		assert_css(&stylesheet, css);
	}
}
