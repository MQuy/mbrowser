use setup::{assert_css, parse};

#[path = "../setup/mod.rs"]
mod setup;

#[test]
pub fn keyword() {
	for prefix in ["scroll", "fixed", "local"].iter() {
		let css = &std::format!(
			r#"
.name {{
	background-attachment: {};
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
	background-attachment: local, local, fixed;
}
    "#;
	let (stylesheet, _) = parse(css);
	assert_css(&stylesheet, css);
}
