use setup::assert_property;

#[macro_use]
#[path = "../setup/mod.rs"]
mod setup;

const TEMPLATE: &str = r#"
.name {{
	column-count: {};
}}"#;

pub fn integer_or_auto_data() -> Vec<(String, String)> {
	let mut data = Vec::with_capacity(1);
	for (input, output) in [("1", "1"), ("2", "2"), ("100", "100"), ("auto", "auto")].iter() {
		data.push((input.to_string(), output.to_string()))
	}
	data
}

test_property!(integer_or_auto, integer_or_auto_data);
