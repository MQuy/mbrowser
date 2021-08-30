use setup::assert_property;
use time::{second_or_milisecond_data, seconds_and_miliseconds_data};

#[macro_use]
#[path = "../setup/mod.rs"]
mod setup;
#[path = "../values/time.rs"]
mod time;

static TEMPLATE: &str = r#"
.name {{
	animation-delay: {};
}}
    "#;

test_property!(second_or_milisecond, second_or_milisecond_data);

test_property!(seconds_and_miliseconds, seconds_and_miliseconds_data);
