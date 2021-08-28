use setup::{assert_css, parse};

mod setup;

#[test]
pub fn parse_page() {
	let css = r#"
    @page {}
    "#;
	let (stylesheet, _) = parse(css);
	assert_css(&stylesheet, css);
}
