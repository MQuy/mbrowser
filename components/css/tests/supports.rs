use setup::{assert_css, parse};

mod setup;

#[test]
pub fn parse_supports_not() {
	let css = r#"
@supports not (display: grid) {
}
    "#;
	let (stylesheet, _) = parse(css);
	assert_css(&stylesheet, css);
}

#[test]
pub fn parse_supports_and() {
	let css = r#"
@supports (display: table-cell) and (display: list-item) {
}
    "#;
	let (stylesheet, _) = parse(css);
	assert_css(&stylesheet, css);
}

#[test]
pub fn parse_supports_or() {
	let css = r#"
@supports (transform-style: preserve) or (-moz-transform-style: preserve) {
}
    "#;
	let (stylesheet, _) = parse(css);
	assert_css(&stylesheet, css);
}

#[test]
pub fn parse_supports_and_or() {
	let css = r#"
@supports ((animation-name: test) and (perspective: 10px)) or (color: red) {
}
    "#;
	let (stylesheet, _) = parse(css);
	assert_css(&stylesheet, css);
}

#[test]
pub fn parse_supports_custom_property() {
	let css = r#"
@supports (--foo: green) {
}
    "#;
	let (stylesheet, _) = parse(css);
	assert_css(&stylesheet, css);
}
