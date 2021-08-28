use setup::{assert_css, parse};

#[path = "../setup/mod.rs"]
mod setup;

#[test]
pub fn keyword() {
	for prefix in ["visible", "hidden"].iter() {
		let css = &std::format!(
			r#"
.name {{
	backface-visibility: {};
}}
    "#,
			prefix
		);
		let (stylesheet, _) = parse(css);
		assert_css(&stylesheet, css);
	}
}
