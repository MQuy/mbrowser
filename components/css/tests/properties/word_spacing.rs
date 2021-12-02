use length::length_percentage_or_normal_data;
use setup::assert_property;

#[path = "../values/length.rs"]
mod length;
#[macro_use]
#[path = "../setup/mod.rs"]
mod setup;

const TEMPLATE: &str = r#"
.name {{
	word-spacing: {};
}}"#;

test_property!(length_percentage_or_normal, length_percentage_or_normal_data);
