use dyn_fmt::AsStrFormatExt;
use number::number_or_percentage_data;
use setup::{assert_css, parse};

#[path = "../values/number.rs"]
mod number;
#[macro_use]
#[path = "../setup/mod.rs"]
mod setup;

const TEMPLATE: &str = r#"
.name {{
	scale: {};
}}"#;

#[test]
pub fn keyword() {
	for input in ["none"].iter() {
		let css = &TEMPLATE.format(&[&input]);
		let (stylesheet, _) = parse(css);
		assert_css(&stylesheet, css);
	}
}

#[test]
pub fn position_angle() {
	let mut values = Vec::with_capacity(1);
	for (input, output) in number_or_percentage_data().iter() {
		values.push((std::format!("{}", input), std::format!("{} {} 1", output, output)));
	}
	for (input, output) in [("10 5.5%", "10 5.5% 1"), ("2.5 100% 0%", "2.5 100% 0%")].iter() {
		values.push((input.to_string(), output.to_string()));
	}
	for (input, output) in values.iter() {
		let cin = &TEMPLATE.format(&[&input]);
		let cout = &TEMPLATE.format(&[&output]);
		let (stylesheet, _) = parse(cin);
		assert_css(&stylesheet, cout);
	}
}
