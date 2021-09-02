use number::number_or_percentage_data;
use setup::assert_property;

#[path = "../values/number.rs"]
mod number;
#[macro_use]
#[path = "../setup/mod.rs"]
mod setup;

const TEMPLATE: &str = r#"
.name {{
	opacity: {};
}}"#;

test_property!(number_or_percentage, number_or_percentage_data);
