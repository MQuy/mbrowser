use common::vector::permutate;

pub fn overflow_data() -> Vec<(String, String)> {
	let mut data = Vec::with_capacity(1);
	for value in ["visible", "hidden", "scroll", "auto"].iter() {
		data.push((value.to_string(), value.to_string()));
	}
	data
}

pub fn resolution_data() -> Vec<(String, String)> {
	let mut data = Vec::with_capacity(1);
	for (value, unit) in permutate(["0.25", "10"].iter(), ["dpi", "dpcm", "dppx", "x"].iter()).iter() {
		let input = std::format!("{}{}", value, unit);
		let output = std::format!("{}{}", value, if **unit == "x" { "dppx" } else { unit });
		data.push((input, output));
	}
	data
}

pub fn box_data() -> Vec<(String, String)> {
	let mut data = Vec::with_capacity(1);
	for value in ["border-box", "padding-box", "content-box"].iter() {
		data.push((value.to_string(), value.to_string()));
	}
	data
}

pub fn line_style_data() -> Vec<(String, String)> {
	let mut data = Vec::with_capacity(1);
	for value in [
		"none", "hidden", "dotted", "dashed", "solid", "double", "groove", "ridge", "inset", "outset",
	]
	.iter()
	{
		data.push((value.to_string(), value.to_string()));
	}
	data
}
