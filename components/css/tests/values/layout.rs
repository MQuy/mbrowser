use common::vector::permutate;

pub fn resolution_data() -> Vec<(String, String)> {
	let mut data = Vec::with_capacity(1);
	for (value, unit) in
		permutate(["0.25", "10"].iter(), ["dpi", "dpcm", "dppx", "x"].iter()).iter()
	{
		let input = std::format!("{}{}", value, unit);
		let output = std::format!("{}{}", value, if **unit == "x" { "dppx" } else { unit });
		data.push((input, output));
	}
	data
}
