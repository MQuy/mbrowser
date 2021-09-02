use number::non_negative_number_data;
use setup::assert_property;

#[path = "../values/number.rs"]
mod number;
#[macro_use]
#[path = "../setup/mod.rs"]
mod setup;

const TEMPLATE: &str = r#"
.name {{
	flex-shrink: {};
}}"#;

test_property!(non_negative_number, non_negative_number_data);
