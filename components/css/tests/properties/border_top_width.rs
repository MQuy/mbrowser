use line::line_width_data;
use setup::assert_property;

#[path = "../values/line.rs"]
mod line;
#[macro_use]
#[path = "../setup/mod.rs"]
mod setup;

const TEMPLATE: &str = r#"
.name {{
	border-top-width: {};
}}"#;

test_property!(line_width, line_width_data);
