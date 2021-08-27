use setup::{assert_stylesheet, parse};

#[path = "../setup/mod.rs"]
mod setup;

#[test]
pub fn keyword() {
	let css = r#"
.name {
	animation-name: none;
}
    "#;
	let (stylesheet, _) = parse(css);
	assert_stylesheet(&stylesheet, css);
}

#[test]
pub fn keyframe_name() {
	for prefix in [
		"nono79",
		"ground-level",
		"-test",
		"_internal",
		"chào",
		"\"hello\"",
	]
	.iter()
	{
		let css = &std::format!(
			r#"
.name {{
	animation-name: {};
}}
    "#,
			prefix
		);
		let (stylesheet, _) = parse(css);
		assert_stylesheet(&stylesheet, css);
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
	assert_stylesheet(&stylesheet, css);
}
