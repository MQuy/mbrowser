use length::non_negative_length_percentage_data;
use setup::assert_property;

#[path = "../values/length.rs"]
mod length;
#[macro_use]
#[path = "../setup/mod.rs"]
mod setup;

const TEMPLATE: &str = r#"
.name {{
	padding-right: {};
}}"#;

test_property!(
	non_negative_length_percentage,
	non_negative_length_percentage_data
);
