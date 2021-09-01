use layout::line_style_data;
use setup::assert_property;

#[path = "../values/layout.rs"]
mod layout;
#[macro_use]
#[path = "../setup/mod.rs"]
mod setup;

const TEMPLATE: &str = r#"
.name {{
	border-block-end-style: {};
}}"#;

test_property!(line_style, line_style_data);
