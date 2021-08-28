use setup::{assert_css, parse};

#[path = "../setup/mod.rs"]
mod setup;

#[test]
pub fn linear() {
	let css = r#"
.name {
	animation-timing-function: linear;
}
    "#;
	let (stylesheet, _) = parse(css);
	assert_css(&stylesheet, css);
}

#[test]
pub fn cubic_bezier_keyword() {
	for prefix in ["ease", "ease-in", "ease-out", "ease-in-out"].iter() {
		let css = &std::format!(
			r#"
.name {{
	animation-timing-function: {};
}}
    "#,
			prefix
		);
		let (stylesheet, _) = parse(css);
		assert_css(&stylesheet, css);
	}
}

#[test]
pub fn cubic_bezier_function() {
	let css = r#"
.name {
	animation-timing-function: cubic-bezier(0, 1.5, 0.25, 5);
}
    "#;
	let (stylesheet, _) = parse(css);
	assert_css(&stylesheet, css)
}

#[test]
pub fn step_keyword() {
	for prefix in ["step-start", "step-end"].iter() {
		let css = &std::format!(
			r#"
.name {{
	animation-timing-function: {};
}}
    "#,
			prefix
		);
		let (stylesheet, _) = parse(css);
		assert_css(&stylesheet, css);
	}
}

#[test]
pub fn step_function() {
	for prefix in [
		", jump-start",
		", jump-end",
		", jump-none",
		", jump-both",
		", start",
		", end",
	]
	.iter()
	{
		let css = &std::format!(
			r#"
.name {{
	animation-timing-function: steps(1.25{});
}}
    "#,
			prefix
		);
		let (stylesheet, _) = parse(css);
		assert_css(&stylesheet, css);
	}
}

#[test]
pub fn step_function_only_steps() {
	let css = r#"
.name {
	animation-timing-function: steps(1.25);
}
    "#;
	let output = r#"
.name {
	animation-timing-function: steps(1.25, end);
}
    "#;
	let (stylesheet, _) = parse(css);
	assert_css(&stylesheet, output);
}
