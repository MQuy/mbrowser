use number::integer_or_auto_data;
use setup::assert_property;

#[path = "../values/number.rs"]
mod number;
#[macro_use]
#[path = "../setup/mod.rs"]
mod setup;

const TEMPLATE: &str = r#"
.name {{
	z-index: {};
}}"#;

test_property!(integer_or_auto, integer_or_auto_data);
