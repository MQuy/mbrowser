use setup::{assert_stylesheet, parse};

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
		assert_stylesheet(&stylesheet, css);
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
		assert_stylesheet(&stylesheet, css);
	}
}

#[test]
pub fn overflow_and_content() {
	for (prefix, content) in ["unsafe", "safe"]
		.iter()
		.zip(["center", "start", "end", "flex-start", "flex-end"].iter())
	{
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
		assert_stylesheet(&stylesheet, css);
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
		assert_stylesheet(&stylesheet, css);
	}
}
