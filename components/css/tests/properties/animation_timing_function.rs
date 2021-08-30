use easing::{
	cubic_bezier_function_data, cubic_bezier_keyword_data, linear_data, step_function_data,
	step_function_only_steps_data, step_keyword_data,
};
use setup::assert_property;

#[path = "../values/easing.rs"]
mod easing;
#[macro_use]
#[path = "../setup/mod.rs"]
mod setup;

const TEMPLATE: &str = r#"
.name {{
	animation-timing-function: {};
}}
    "#;

test_property!(linear, linear_data);

test_property!(cubic_bezier_keyword, cubic_bezier_keyword_data);

test_property!(cubic_bezier_function, cubic_bezier_function_data);

test_property!(step_keyword, step_keyword_data);

test_property!(step_function, step_function_data);

test_property!(step_function_only_steps, step_function_only_steps_data);
