pub fn border_corner_radius_data() -> Vec<(String, String)> {
	let mut data = Vec::with_capacity(1);
	for (input, output) in [("0px", "0px 0px"), ("5.5px 12.5%", "5.5px 12.5%")].iter() {
		data.push((input.to_string(), output.to_string()))
	}
	data
}
