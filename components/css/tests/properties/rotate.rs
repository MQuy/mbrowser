use angle::angle_data;
use common::vector::permutate;
use css::str::join_strings;
use dyn_fmt::AsStrFormatExt;
use setup::{assert_css, assert_property, parse};

#[path = "../values/angle.rs"]
mod angle;
#[macro_use]
#[path = "../setup/mod.rs"]
mod setup;

const TEMPLATE: &str = r#"
.name {{
	rotate: {};
}}"#;

#[test]
pub fn keyword() {
	for input in ["none"].iter() {
		let css = &TEMPLATE.format(&[&input]);
		let (stylesheet, _) = parse(css);
		assert_css(&stylesheet, css);
	}
}

test_property!(angle, angle_data);

#[test]
pub fn position_angle() {
	for (position, (angle_input, angle_output)) in
		permutate(["x", "y", "z", "1 0.25 10"].iter(), angle_data().iter()).iter()
	{
		let input = join_strings(vec![position, angle_input], " ");
		let output = join_strings(vec![position, angle_output], " ");
		let cin = &TEMPLATE.format(&[&input]);
		let cout = &TEMPLATE.format(&[&output]);
		let (stylesheet, _) = parse(cin);
		assert_css(&stylesheet, cout);
	}
}
