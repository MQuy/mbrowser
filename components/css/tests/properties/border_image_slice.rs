use dyn_fmt::AsStrFormatExt;
use number::non_negative_number_or_percentage_rect_data;
use setup::{assert_css, assert_property, parse};

#[path = "../values/number.rs"]
mod number;
#[macro_use]
#[path = "../setup/mod.rs"]
mod setup;

const TEMPLATE: &str = r#"
.name {{
	border-image-slice: {};
}}"#;

test_property!(rect, non_negative_number_or_percentage_rect_data);

#[test]
pub fn rect_and_fill() {
	for (input, output) in [
		("0.25 fill", "0.25 0.25 0.25 0.25 fill"),
		("fill 1% 5", "1% 5 1% 5 fill"),
	]
	.iter()
	{
		let cin = &TEMPLATE.format(&[&input]);
		let cout = &TEMPLATE.format(&[&output]);
		let (stylesheet, _) = parse(cin);
		assert_css(&stylesheet, cout);
	}
}
