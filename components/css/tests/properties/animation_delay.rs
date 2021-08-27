use setup::{assert_stylesheet, parse};

#[path = "../setup/mod.rs"]
mod setup;

#[test]
pub fn second_or_milisecond() {
	for (value, unit) in [0, 1, 500000].iter().zip(["s", "ms"].iter()) {
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
		assert_stylesheet(&stylesheet, css);
	}

	for (value, unit) in [0.25, 1.5, 500.000].iter().zip(["s", "ms"].iter()) {
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
		assert_stylesheet(&stylesheet, css);
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
	assert_stylesheet(&stylesheet, css);
}
