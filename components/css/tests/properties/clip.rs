use length::length_or_auto_rect_auto_data;
use setup::assert_property;

#[path = "../values/length.rs"]
mod length;
#[macro_use]
#[path = "../setup/mod.rs"]
mod setup;

const TEMPLATE: &str = r#"
.name {{
	clip: rect({});
}}"#;

test_property!(length_or_auto_rect_auto, length_or_auto_rect_auto_data);
