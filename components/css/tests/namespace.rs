use setup::{assert_stylesheet, parse};

mod setup;

#[test]
pub fn parse_namespace_with_string() {
    let css = r#"
@namespace toto "http://toto.example.org";
@namespace "http://example.com/foo";
"#;
    let (stylesheet, _) = parse(css);
    assert_stylesheet(&stylesheet, css);
}

#[test]
pub fn parse_namespace_with_url() {
    let css = r#"
@namespace url(http://www.w3.org/1999/xhtml);
@namespace svg url(http://www.w3.org/2000/svg);"#;
    let (stylesheet, _) = parse(css);
    assert_stylesheet(&stylesheet, css);
}
