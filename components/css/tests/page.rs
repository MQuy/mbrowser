use setup::{assert_stylesheet, parse};

mod setup;

#[test]
pub fn parse_page() {
	let css = r#"
    @page {}
    "#;
	let (stylesheet, _) = parse(css);
	assert_stylesheet(&stylesheet, css);
}
