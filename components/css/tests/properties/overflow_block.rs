use layout::overflow_data;
use setup::assert_property;

#[path = "../values/layout.rs"]
mod layout;
#[macro_use]
#[path = "../setup/mod.rs"]
mod setup;

const TEMPLATE: &str = r#"
.name {{
	overflow-block: {};
}}"#;

test_property!(keyword, overflow_data);
