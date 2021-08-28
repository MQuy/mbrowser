use common::vector::permutate;
use setup::{assert_css, parse};

#[path = "../setup/mod.rs"]
mod setup;

#[test]
pub fn second_or_milisecond() {
	for (value, unit) in permutate(
		["0", "1", "500000", "0.25", "1.5"].iter(),
		["s", "ms"].iter(),
	) {
		let css = &std::format!(
			r#"
.name {{
	animation-delay: {}{};
}}
    "#,
			value,
			unit
		);
		let (stylesheet, _) = parse(css);
		assert_css(&stylesheet, css);
	}
}

#[test]
pub fn mixed_seconds_and_miliseconds() {
	let css = &std::format!(
		r#"
.name {{
	animation-delay: 0s, 1.5s, 30ms;
}}
    "#,
	);
	let (stylesheet, _) = parse(css);
	assert_css(&stylesheet, css);
}
