use border::border_corner_radius_data;
use setup::assert_property;

#[path = "../values/border.rs"]
mod border;
#[macro_use]
#[path = "../setup/mod.rs"]
mod setup;

const TEMPLATE: &str = r#"
.name {{
	border-end-start-radius: {};
}}"#;

test_property!(border_corner_radius, border_corner_radius_data);
