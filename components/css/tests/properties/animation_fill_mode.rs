use setup::{assert_css, parse};

#[path = "../setup/mod.rs"]
mod setup;

#[test]
pub fn keyword() {
	for prefix in ["none", "forwards", "backwards", "both"].iter() {
		let css = &std::format!(
			r#"
.name {{
	animation-fill-mode: {};
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
	animation-fill-mode: both, none, backwards;
}
    "#;
	let (stylesheet, _) = parse(css);
	assert_css(&stylesheet, css);
}
