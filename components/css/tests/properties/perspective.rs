use length::non_negative_length_or_none_data;
use setup::assert_property;

#[path = "../values/length.rs"]
mod length;
#[macro_use]
#[path = "../setup/mod.rs"]
mod setup;

const TEMPLATE: &str = r#"
.name {{
	perspective: {};
}}"#;

test_property!(non_negative_length_or_none, non_negative_length_or_none_data);
