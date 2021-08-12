use crate::setup::{assert_stylesheet, parse};

mod setup;

#[test]
pub fn parse_media_empty_all() {
    let css = r#"
@media all {
}
"#;
    let (stylesheet, _) = parse(css);
    assert_stylesheet(&stylesheet, css);
}

#[test]
pub fn parse_media_not_screen_with_and() {
    let css = r#"
@media not screen and (min-width: 900px) {
}
"#;
    let (stylesheet, _) = parse(css);
    assert_stylesheet(&stylesheet, css);
}

#[test]
pub fn parse_media_and_or_with_keyword() {
    let css = r#"
@media ((orientation: portrait) and (hover: none)) or (color: 8) {
}
"#;
    let (stylesheet, _) = parse(css);
    assert_stylesheet(
        &stylesheet,
        r#"
@media ((orientation = portrait) and (hover = none)) or (color = 8) {
}
"#,
    );
}

#[test]
pub fn parse_media_with_range() {
    let css = r#"
@media (640px <= height <= 900px) {
}
"#;
    let (stylesheet, _) = parse(css);
    assert_stylesheet(
        &stylesheet,
        r#"
@media (height >= 640px and height <= 900px) {
}
"#,
    );
}

#[test]
pub fn parse_media_negative_feature() {
    let css = r#"
@media (not (color)) or (hover) {
}
"#;
    let (stylesheet, _) = parse(css);
    assert_stylesheet(&stylesheet, css);
}

#[test]
pub fn parse_media_multiple_queries() {
    let css = r#"
@media (min-height: 680px), screen and (orientation: portrait) {
}
"#;
    let (stylesheet, _) = parse(css);
    assert_stylesheet(
        &stylesheet,
        r#"
@media (min-height: 680px), screen and (orientation = portrait) {
}
"#,
    );
}
