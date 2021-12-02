use length::non_negative_length_or_auto_data;
use setup::assert_property;

#[path = "../values/length.rs"]
mod length;
#[macro_use]
#[path = "../setup/mod.rs"]
mod setup;

const TEMPLATE: &str = r#"
.name {{
	column-width: {};
}}"#;

test_property!(non_negative_length_or_auto, non_negative_length_or_auto_data);
