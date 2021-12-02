use counter::{counter_with_integer_and_none_data, reversed_counter_with_integer_and_none_data};
use setup::assert_property;

#[path = "../values/counter.rs"]
mod counter;
#[macro_use]
#[path = "../setup/mod.rs"]
mod setup;

const TEMPLATE: &str = r#"
.name {{
	counter-reset: {};
}}"#;

test_property!(counter_with_integer_and_none, counter_with_integer_and_none_data);

test_property!(
	reversed_counter_with_integer_and_none,
	reversed_counter_with_integer_and_none_data
);
