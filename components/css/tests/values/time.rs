use common::vector::permutate;

pub fn second_or_milisecond_data() -> Vec<(String, String)> {
	let mut data = Vec::with_capacity(1);

	for (value, unit) in permutate(
		["0", "1", "500000", "0.25", "1.5"].iter(),
		["s", "ms"].iter(),
	) {
		let input = std::format!("{}{}", value, unit);
		let output = input.clone();
		data.push((input, output));
	}
	data
}

pub fn seconds_and_miliseconds_data() -> Vec<(String, String)> {
	let mut data = Vec::with_capacity(1);
	for value in ["0s, 1.5s, 30ms"].iter() {
		let input = value.to_string();
		let output = input.clone();
		data.push((input, output));
	}
	data
}
