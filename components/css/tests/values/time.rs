use common::vector::permutate;
use css::values::time::Time;
use setup::{assert_css, parse_value};

#[path = "../setup/mod.rs"]
mod setup;

#[test]
pub fn second_or_milisecond() {
	for (value, unit) in permutate(
		["0", "1", "500000", "0.25", "1.5"].iter(),
		["s", "ms"].iter(),
	) {
		let css = &std::format!("{}{}", value, unit);
		let value = parse_value(css, Time::parse).unwrap();
		assert_css(&value, css);
	}
}
