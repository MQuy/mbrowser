use setup::{assert_css, parse};

#[path = "../setup/mod.rs"]
mod setup;

#[test]
pub fn keyword() {
	for prefix in ["running", "paused"].iter() {
		let css = &std::format!(
			r#"
.name {{
	animation-play-state: {};
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
	animation-play-state: running, running, paused;
}
    "#;
	let (stylesheet, _) = parse(css);
	assert_css(&stylesheet, css);
}
