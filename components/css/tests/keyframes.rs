use setup::{assert_stylesheet, parse};

mod setup;

#[test]
pub fn parse_keyframes_string_from_to() {
	let css = r#"@keyframes "foo" { from {} to {} }"#;
	let (stylesheet, _) = parse(css);
	assert_stylesheet(
		&stylesheet,
		r#"
@keyframes foo {
0% {
}
100% {
}
}
    "#,
	);
}

#[test]
pub fn parse_keyframes_duplicate_in_selectors() {
	let css = r#"
@keyframes identifier {
0% {
}
50%, 50% {
}
100% {
}
}
    "#;
	let (stylesheet, _) = parse(css);
	assert_stylesheet(
		&stylesheet,
		r#"
@keyframes identifier {
0% {
}
50% {
}
100% {
}
}
        "#,
	);
}
