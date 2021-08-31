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
