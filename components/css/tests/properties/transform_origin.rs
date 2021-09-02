use position::transform_origin_data;
use setup::assert_property;

#[path = "../values/position.rs"]
mod position;
#[macro_use]
#[path = "../setup/mod.rs"]
mod setup;

const TEMPLATE: &str = r#"
.name {{
	transform-origin: {};
}}"#;

test_property!(position, transform_origin_data);
