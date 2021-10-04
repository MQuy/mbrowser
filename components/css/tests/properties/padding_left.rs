use length::non_negative_length_percentage_or_auto_data;
use setup::assert_property;

#[path = "../values/length.rs"]
mod length;
#[macro_use]
#[path = "../setup/mod.rs"]
mod setup;

const TEMPLATE: &str = r#"
.name {{
	padding-left: {};
}}"#;

test_property!(
	non_negative_length_percentage_or_auto,
	non_negative_length_percentage_or_auto_data
);
