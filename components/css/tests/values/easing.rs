pub fn linear_data() -> Vec<(String, String)> {
	let input = "linear".to_string();
	let output = input.clone();
	vec![(input, output)]
}

pub fn cubic_bezier_keyword_data() -> Vec<(String, String)> {
	let mut data = Vec::with_capacity(1);
	for prefix in ["ease", "ease-in", "ease-out", "ease-in-out"].iter() {
		let input = prefix.to_string();
		let output = input.clone();
		data.push((input, output));
	}
	data
}

pub fn cubic_bezier_function_data() -> Vec<(String, String)> {
	let input = "cubic-bezier(0, 1.5, 0.25, 5)".to_string();
	let output = input.clone();
	vec![(input, output)]
}

pub fn step_keyword_data() -> Vec<(String, String)> {
	let mut data = Vec::with_capacity(1);
	for prefix in ["step-start", "step-end"].iter() {
		let input = prefix.to_string();
		let output = input.clone();
		data.push((input, output));
	}
	data
}

pub fn step_function_data() -> Vec<(String, String)> {
	let mut data = Vec::with_capacity(1);
	for prefix in [
		", jump-start",
		", jump-end",
		", jump-none",
		", jump-both",
		", start",
		", end",
	]
	.iter()
	{
		let input = std::format!("steps(1.25{})", prefix);
		let output = input.clone();
		data.push((input, output));
	}
	data
}

pub fn step_function_only_steps_data() -> Vec<(String, String)> {
	let input = "steps(1.25)".to_string();
	let output = "steps(1.25, end)".to_string();
	vec![(input, output)]
}
