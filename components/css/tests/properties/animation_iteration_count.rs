use setup::{assert_css, parse};

#[path = "../setup/mod.rs"]
mod setup;

#[test]
pub fn keyword() {
	let css = r#"
.name {
	animation-iteration-count: infinite;
}
    "#;
	let (stylesheet, _) = parse(css);
	assert_css(&stylesheet, css);
}

#[test]
pub fn number() {
	for value in ["0", "1", "500000", "0.25", "1.5"].iter() {
		let css = &std::format!(
			r#"
.name {{
	animation-iteration-count: {};
}}
    "#,
			value,
		);
		let (stylesheet, _) = parse(css);
		assert_css(&stylesheet, css);
	}
}

#[test]
pub fn mixed_keyword_and_number() {
	let css = r#"
.name {
	animation-iteration-count: infinite, 0.25, 5;
}
    "#;
	let (stylesheet, _) = parse(css);
	assert_css(&stylesheet, css);
}
