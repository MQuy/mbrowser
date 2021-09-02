use setup::assert_property;
use time::{second_or_milisecond_data, seconds_and_miliseconds_data};

#[path = "../values/time.rs"]
mod time;
#[macro_use]
#[path = "../setup/mod.rs"]
mod setup;

const TEMPLATE: &str = r#"
.name {{
	transition-delay: {};
}}"#;

test_property!(second_or_milisecond, second_or_milisecond_data);

test_property!(seconds_and_miliseconds, seconds_and_miliseconds_data);
