use setup::assert_property;
use transform::transform_data;

#[path = "../values/transform.rs"]
mod transform;
#[macro_use]
#[path = "../setup/mod.rs"]
mod setup;

const TEMPLATE: &str = r#"
.name {{
	transform: {};
}}"#;

test_property!(function, transform_data);
