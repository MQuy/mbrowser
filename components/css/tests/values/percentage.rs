use common::vector::permutate;

pub fn ratio_data() -> Vec<(String, String)> {
	let mut data = Vec::with_capacity(1);
	for (first, second) in
		permutate(["0", "2.5", "10"].iter(), ["500", "1000.25", ""].iter()).iter()
	{
		let isecond = if second.len() == 0 {
			"".to_string()
		} else {
			std::format!(" / {}", second)
		};
		let input = std::format!("{}{}", first, isecond);
		let osecond = if second.len() == 0 { "1" } else { second };
		let output = std::format!("{} / {}", first, osecond);
		data.push((input, output));
	}
	data
}
