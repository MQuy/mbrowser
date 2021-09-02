use length::size_data;
use setup::assert_property;

#[path = "../values/length.rs"]
mod length;
#[macro_use]
#[path = "../setup/mod.rs"]
mod setup;

const TEMPLATE: &str = r#"
.name {{
	max-block-size: {};
}}"#;

test_property!(size, size_data);
