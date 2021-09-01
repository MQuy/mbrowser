use counter::counter_with_integer_and_none_data;
use setup::assert_property;

#[path = "../values/counter.rs"]
mod counter;
#[macro_use]
#[path = "../setup/mod.rs"]
mod setup;

const TEMPLATE: &str = r#"
.name {{
	counter-set: {};
}}"#;

test_property!(
	counter_with_integer_and_none,
	counter_with_integer_and_none_data
);
