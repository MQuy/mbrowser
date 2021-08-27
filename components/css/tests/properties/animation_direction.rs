use setup::{assert_stylesheet, parse};

#[path = "../setup/mod.rs"]
mod setup;

#[test]
pub fn keyword() {
	for prefix in ["normal", "reverse", "alternate", "alternate-reverse"].iter() {
		let css = &std::format!(
			r#"
.name {{
	animation-direction: {};
}}
    "#,
			prefix
		);
		let (stylesheet, _) = parse(css);
		assert_stylesheet(&stylesheet, css);
	}
}

#[test]
pub fn keywords() {
	let css = r#"
.name {
	animation-direction: normal, reverse, alternate;
}
    "#;
	let (stylesheet, _) = parse(css);
	assert_stylesheet(&stylesheet, css);
}
