use setup::{assert_css, parse};

#[path = "../setup/mod.rs"]
mod setup;

#[test]
pub fn keyword() {
	for prefix in ["border-box", "padding-box", "content-box"].iter() {
		let css = &std::format!(
			r#"
.name {{
	background-clip: {};
}}
    "#,
			prefix
		);
		let (stylesheet, _) = parse(css);
		assert_css(&stylesheet, css);
	}
}

#[test]
pub fn keywords() {
	let css = r#"
.name {
	background-clip: padding-box, border-box, padding-box;
}
    "#;
	let (stylesheet, _) = parse(css);
	assert_css(&stylesheet, css);
}
