use layout::box_data;
use setup::{assert_css, assert_property, parse};

#[path = "../values/layout.rs"]
mod layout;
#[macro_use]
#[path = "../setup/mod.rs"]
mod setup;

const TEMPLATE: &str = r#"
.name {{
	background-origin: {};
}}"#;

test_property!(keyword, box_data);

#[test]
pub fn keywords() {
	let css = r#"
.name {
	background-origin: border-box, content-box, content-box, padding-box;
}
    "#;
	let (stylesheet, _) = parse(css);
	assert_css(&stylesheet, css);
}
