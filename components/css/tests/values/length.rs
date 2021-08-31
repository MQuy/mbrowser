use percentage::percentage_data;
#[path = "./percentage.rs"]
mod percentage;

pub fn length_data() -> Vec<(String, String)> {
	let mut data = Vec::with_capacity(1);
	for value in ["1px"].iter() {
		data.push((value.to_string(), value.to_string()));
	}
	data
}

pub fn length_percentage_data() -> Vec<(String, String)> {
	let mut length = length_data();
	let mut percentage = percentage_data();
	length.append(&mut percentage);
	length
}
