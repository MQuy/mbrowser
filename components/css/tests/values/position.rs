use common::vector::permutate;
use css::str::join_strings;

pub fn position_data() -> Vec<(String, String)> {
	let mut data = Vec::with_capacity(1);
	for (horizontal_input, vertical_input) in permutate(
		["left", "center", "right", ""].iter(),
		["top", "center", "bottom", ""].iter(),
	)
	.iter()
	{
		if horizontal_input.len() == 0 && vertical_input.len() == 0 {
			continue;
		}
		let horizontal_output = if horizontal_input.len() == 0 {
			"center"
		} else {
			horizontal_input
		};
		let vertical_output = if vertical_input.len() == 0 {
			"center"
		} else {
			vertical_input
		};
		let input = join_strings(vec![horizontal_input, vertical_input], " ");
		let output = std::format!("{} {}", horizontal_output, vertical_output);
		data.push((input, output));
	}
	for (horizontal_input, vertical_input) in permutate(
		["left", "center", "right", "10px", "5%"].iter(),
		["top", "center", "bottom", "5.5px", "0.25%", ""].iter(),
	)
	.iter()
	{
		let vertical_output = if vertical_input.len() == 0 {
			"center"
		} else {
			vertical_input
		};
		let input = join_strings(vec![horizontal_input, vertical_input], " ");
		let output = std::format!("{} {}", horizontal_input, vertical_output);
		data.push((input, output));
	}
	for (horizontal, vertical) in permutate(
		["left 10px", "right 5%"].iter(),
		["top 0.5%", "bottom 5.5px"].iter(),
	)
	.iter()
	{
		let value = std::format!("{} {}", horizontal, vertical);
		data.push((value.to_string(), value.to_string()));
	}
	data
}

pub fn transform_origin_data() -> Vec<(String, String)> {
	let mut data = Vec::with_capacity(1);
	for horizontal in [
		"left", "center", "right", "top", "bottom", "10px", "0.25px", "50%",
	]
	.iter()
	{
		let input = std::format!("{}", horizontal);
		let output = std::format!("{} {} {}", horizontal, "center", "0px");
		data.push((input, output));
	}
	for ((horizontal, vertical), z) in permutate(
		permutate(
			["left", "center", "right", "10px", "5%"].iter(),
			["top", "center", "bottom", "0.5px", "2.5%"].iter(),
		)
		.iter(),
		["1px", ""].iter(),
	)
	.iter()
	{
		let input = std::format!(
			"{} {}{}",
			horizontal,
			vertical,
			if z.len() == 0 {
				"".to_string()
			} else {
				std::format!(" {}", z)
			}
		);
		let output = std::format!(
			"{} {} {}",
			horizontal,
			vertical,
			if z.len() == 0 { "0px" } else { z }
		);
		data.push((input, output));
	}
	for ((horizontal, vertical), z) in permutate(
		permutate(
			["center", "left", "right"].iter(),
			["center", "top", "bottom"].iter(),
		)
		.iter(),
		["0.5px", ""].iter(),
	)
	.iter()
	{
		let input = std::format!(
			"{} {}{}",
			horizontal,
			vertical,
			if z.len() == 0 {
				"".to_string()
			} else {
				std::format!(" {}", z)
			}
		);
		let reversed_input = std::format!(
			"{} {}{}",
			vertical,
			horizontal,
			if z.len() == 0 {
				"".to_string()
			} else {
				std::format!(" {}", z)
			}
		);
		let output = std::format!(
			"{} {} {}",
			horizontal,
			vertical,
			if z.len() == 0 { "0px" } else { z }
		);
		data.push((input, output.clone()));
		data.push((reversed_input, output));
	}
	data
}
