use length::non_negative_length_data;

#[path = "./length.rs"]
mod length;

pub fn line_width_data() -> Vec<(String, String)> {
	let mut data = Vec::with_capacity(1);
	for (input, _) in non_negative_length_data().iter() {
		data.push((input.to_string(), input.to_string()));
	}
	for value in ["thin", "medium", "thick"].iter() {
		data.push((value.to_string(), value.to_string()));
	}
	data
}
