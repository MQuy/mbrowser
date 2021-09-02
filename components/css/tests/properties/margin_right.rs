use length::length_percentage_or_auto_data;
use setup::assert_property;

#[path = "../values/length.rs"]
mod length;
#[macro_use]
#[path = "../setup/mod.rs"]
mod setup;

const TEMPLATE: &str = r#"
.name {{
	margin-right: {};
}}"#;

test_property!(length_percentage_or_auto, length_percentage_or_auto_data);
