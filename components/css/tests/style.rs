use setup::{assert_stylesheet, parse};

mod setup;

#[test]
pub fn parse_keyframes_string_from_to() {
    let css = r#"
.name {
    display: block;
}
"#;
    let (stylesheet, _) = parse(css);
    assert_stylesheet(
        &stylesheet,
        r#"
.name {
	display: block;
}
    "#,
    );
}
