use common::vector::permutate;

pub fn angle_data() -> Vec<(String, String)> {
	let mut data = Vec::with_capacity(1);
	for (value, unit) in permutate(
		["0", "5.5", "360"].iter(),
		["deg", "grad", "rad", "turn"].iter(),
	)
	.iter()
	{
		let input = std::format!("{}{}", value, unit);
		data.push((input.to_string(), input.to_string()));
	}
	data
}

pub fn angle_or_zero_data() -> Vec<(String, String)> {
	let mut data = angle_data();
	data.push(("0".to_string(), "0".to_string()));
	data
}
