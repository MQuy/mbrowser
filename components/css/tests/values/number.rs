use percentage::percentage_data;

#[path = "./percentage.rs"]
mod percentage;

pub fn number_data() -> Vec<(String, String)> {
	let mut data = Vec::with_capacity(1);
	for (input, output) in [
		("0", "0"),
		("-1.25", "-1.25"),
		(".5", "0.5"),
		("100", "100"),
	]
	.iter()
	{
		data.push((input.to_string(), output.to_string()))
	}
	data
}

pub fn number_or_auto_data() -> Vec<(String, String)> {
	let mut data = number_data();
	data.push(("auto".to_string(), "auto".to_string()));
	data
}

pub fn non_negative_number_data() -> Vec<(String, String)> {
	let mut data = Vec::with_capacity(1);
	for (input, output) in [("0", "0"), ("1.25", "1.25"), (".5", "0.5"), ("100", "100")].iter() {
		data.push((input.to_string(), output.to_string()))
	}
	data
}

pub fn non_negative_number_or_percentage_data() -> Vec<(String, String)> {
	let mut data = non_negative_number_data();
	let mut percentage = percentage_data();
	data.append(&mut percentage);
	data
}

pub fn non_negative_number_or_percentage_rect_data() -> Vec<(String, String)> {
	let mut data = Vec::with_capacity(1);
	for (input, output) in non_negative_number_or_percentage_data().iter() {
		data.push((
			input.to_string(),
			std::format!("{} {} {} {}", output, output, output, output),
		));
	}

	for (input, output) in [
		("10% 5", "10% 5 10% 5"),
		("1% 2.5% 5%", "1% 2.5% 5% 2.5%"),
		("1 200 .25 1.5", "1 200 0.25 1.5"),
	]
	.iter()
	{
		data.push((input.to_string(), output.to_string()));
	}

	data
}
