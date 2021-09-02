use position::position_data;
use setup::assert_property;

#[path = "../values/position.rs"]
mod position;
#[macro_use]
#[path = "../setup/mod.rs"]
mod setup;

const TEMPLATE: &str = r#"
.name {{
	perspective-origin: {};
}}"#;

test_property!(position, position_data);
