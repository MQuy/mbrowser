use setup::{assert_stylesheet, parse};

#[path = "../setup/mod.rs"]
mod setup;

#[test]
pub fn parse_page() {
    let css = r#"
.xxx {
	aspect-ratio: auto;
}
    "#;
    let (stylesheet, _) = parse(css);
    assert_stylesheet(&stylesheet, css);
}
