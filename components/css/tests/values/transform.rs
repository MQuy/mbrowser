use angle::angle_or_zero_data;
use common::vector::permutate;

#[path = "./angle.rs"]
mod angle;

pub fn transform_data() -> Vec<(String, String)> {
	let mut data = Vec::with_capacity(1);
	for value in ["0 0.5 3 100 -1 -2.5"].iter() {
		let input = std::format!("matrix({})", value);
		data.push((input.to_string(), input.to_string()));
	}
	for (input, output) in [
		("10px", "10px, 0px"),
		("5.5%, .25px", "5.5%, 0.25px"),
		("0%, 100%", "0%, 100%"),
	]
	.iter()
	{
		data.push((
			std::format!("translate({})", input),
			std::format!("translate({})", output),
		));
	}
	for (name, value) in permutate(["translateX", "translateY"].iter(), ["0px", "1.5px", "10%"].iter()).iter() {
		let input = std::format!("{}({})", name, value);
		data.push((input.to_string(), input.to_string()));
	}
	for (input, output) in [("-10", "-10, -10"), ("-1, 2.5", "-1, 2.5"), ("0.25, 30", "0.25, 30")].iter() {
		data.push((std::format!("scale({})", input), std::format!("scale({})", output)));
	}
	for (name, value) in permutate(["scaleX", "scaleY"].iter(), ["-2", "0", "0.5", "8"].iter()).iter() {
		let input = std::format!("{}({})", name, value);
		data.push((input.to_string(), input.to_string()));
	}
	for (input, output) in angle_or_zero_data().iter() {
		data.push((std::format!("rotate({})", input), std::format!("rotate({})", output)));
	}
	for (input, output) in angle_or_zero_data().iter() {
		data.push((std::format!("skew({})", input), std::format!("skew({}, 0)", output)));
	}
	data.push(("skew(0.25deg, 1.5deg)".to_string(), "skew(0.25deg, 1.5deg)".to_string()));
	for (name, (input, output)) in permutate(["skewX", "skewY"].iter(), angle_or_zero_data().iter()).iter() {
		data.push((
			std::format!("{}({})", name, input),
			std::format!("{}({})", name, output),
		));
	}

	data
}
