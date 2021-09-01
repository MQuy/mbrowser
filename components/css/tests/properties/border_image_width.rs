use length::non_negative_length_percentage_or_number_rect_data;
use setup::assert_property;

#[path = "../values/length.rs"]
mod length;
#[macro_use]
#[path = "../setup/mod.rs"]
mod setup;

const TEMPLATE: &str = r#"
.name {{
	border-image-width: {};
}}"#;

test_property!(rect, non_negative_length_percentage_or_number_rect_data);
