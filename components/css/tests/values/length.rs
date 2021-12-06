use number::non_negative_number_data;
use percentage::percentage_data;

#[path = "./number.rs"]
mod number;
#[path = "./percentage.rs"]
mod percentage;

pub fn length_data() -> Vec<(String, String)> {
	let mut data = Vec::with_capacity(1);
	for value in ["1px"].iter() {
		data.push((value.to_string(), value.to_string()));
	}
	data
}

pub fn length_or_auto_data() -> Vec<(String, String)> {
	let mut length = length_data();
	length.push(("auto".to_string(), "auto".to_string()));
	length
}

pub fn non_negative_length_data() -> Vec<(String, String)> {
	let mut data = Vec::with_capacity(1);
	for value in ["1px"].iter() {
		data.push((value.to_string(), value.to_string()));
	}
	data
}

pub fn non_negative_length_or_none_data() -> Vec<(String, String)> {
	let mut length = non_negative_length_data();
	length.push(("none".to_string(), "none".to_string()));
	length
}

pub fn non_negative_length_or_auto_data() -> Vec<(String, String)> {
	let mut length = non_negative_length_data();
	length.push(("auto".to_string(), "auto".to_string()));
	length
}

pub fn length_percentage_data() -> Vec<(String, String)> {
	let mut length = length_data();
	let mut percentage = percentage_data();
	length.append(&mut percentage);
	length
}

pub fn length_percentage_or_auto_data() -> Vec<(String, String)> {
	let mut length = length_percentage_data();
	length.push(("auto".to_string(), "auto".to_string()));
	length
}

pub fn length_percentage_or_normal_data() -> Vec<(String, String)> {
	let mut length = length_data();
	let mut percentage = percentage_data();
	length.append(&mut percentage);
	length.push(("normal".to_string(), "normal".to_string()));
	length
}

pub fn non_negative_length_percentage_data() -> Vec<(String, String)> {
	let mut length = non_negative_length_data();
	let mut percentage = percentage_data();
	length.append(&mut percentage);
	length
}

pub fn non_negative_length_percentage_or_auto_data() -> Vec<(String, String)> {
	let mut length = non_negative_length_data();
	let mut percentage = percentage_data();
	length.append(&mut percentage);
	length.push(("auto".to_string(), "auto".to_string()));
	length
}

pub fn non_negative_length_percentage_number_or_normal_data() -> Vec<(String, String)> {
	let mut length = non_negative_length_percentage_data();
	let mut number = non_negative_number_data();
	length.append(&mut number);
	length.push(("normal".to_string(), "normal".to_string()));
	length
}

pub fn size_data() -> Vec<(String, String)> {
	let mut data = length_percentage_data();
	for value in [
		"auto",
		"min-content",
		"max-content",
		"fit-content(10px)",
		"fit-content(5.5%)",
	]
	.iter()
	{
		data.push((value.to_string(), value.to_string()));
	}
	data
}

pub fn max_size_data() -> Vec<(String, String)> {
	let mut data = length_percentage_data();
	for value in [
		"none",
		"min-content",
		"max-content",
		"fit-content(10px)",
		"fit-content(5.5%)",
	]
	.iter()
	{
		data.push((value.to_string(), value.to_string()));
	}
	data
}

pub fn non_negative_length_or_number_rect_data() -> Vec<(String, String)> {
	let mut data = Vec::with_capacity(1);
	for (input, output) in length_data().iter() {
		data.push((
			input.to_string(),
			std::format!("{} {} {} {}", output, output, output, output),
		));
	}
	for (input, output) in non_negative_number_data().iter() {
		data.push((
			input.to_string(),
			std::format!("{} {} {} {}", output, output, output, output),
		));
	}

	for (input, output) in [
		("10px 5", "10px 5 10px 5"),
		("1px 2.5px 5px", "1px 2.5px 5px 2.5px"),
		("1 200 .25 1.5", "1 200 0.25 1.5"),
	]
	.iter()
	{
		data.push((input.to_string(), output.to_string()));
	}

	data
}

pub fn non_negative_length_percentage_or_number_rect_data() -> Vec<(String, String)> {
	let mut data = Vec::with_capacity(1);
	for (input, output) in length_percentage_data().iter() {
		data.push((
			input.to_string(),
			std::format!("{} {} {} {}", output, output, output, output),
		));
	}
	for (input, output) in non_negative_number_data().iter() {
		data.push((
			input.to_string(),
			std::format!("{} {} {} {}", output, output, output, output),
		));
	}

	for (input, output) in [
		("10px 5", "10px 5 10px 5"),
		("1px 2.5px 5px", "1px 2.5px 5px 2.5px"),
		("1 200 .25 1.5", "1 200 0.25 1.5"),
	]
	.iter()
	{
		data.push((input.to_string(), output.to_string()));
	}

	data
}

pub fn length_or_auto_rect_auto_data() -> Vec<(String, String)> {
	let mut data = Vec::with_capacity(1);
	for (input, output) in length_or_auto_data().iter() {
		data.push((
			std::format!("{}, {}, {}, {}", input, input, input, input),
			std::format!("{}, {}, {}, {}", output, output, output, output),
		));
	}

	for value in ["10px, auto, 2.5px, auto"].iter() {
		data.push((value.to_string(), value.to_string()));
	}

	data
}

pub fn non_negative_length_percentage_or_normal_data() -> Vec<(String, String)> {
	let mut length = non_negative_length_data();
	let mut percentage = percentage_data();
	length.append(&mut percentage);
	length.push(("normal".to_string(), "normal".to_string()));
	length
}
