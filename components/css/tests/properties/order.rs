use number::integer_data;
use setup::assert_property;

#[path = "../values/number.rs"]
mod number;
#[macro_use]
#[path = "../setup/mod.rs"]
mod setup;

const TEMPLATE: &str = r#"
.name {{
	order: {};
}}"#;

test_property!(integer, integer_data);
