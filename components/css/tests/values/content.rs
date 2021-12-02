use common::vector::permutate;

pub fn counter_style_data() -> Vec<(String, String)> {
	let mut data = Vec::with_capacity(1);
	data.push(("echo".to_string(), "echo".to_string()));
	for (symbol_type, symbol_name) in permutate(
		["cyclic", "numeric", "alphabetic", "symbolic", "fixed", ""].iter(),
		["\"something\"", "element(#id)", "element(#id) element(#class)"].iter(),
	)
	.iter()
	{
		let value = std::format!(
			"symbols({}{})",
			if symbol_type.len() == 0 {
				"".to_string()
			} else {
				std::format!("{} ", symbol_type)
			},
			symbol_name
		);
		data.push((value.to_string(), value.to_string()));
	}
	data
}

pub fn counter_data() -> Vec<(String, String)> {
	let mut data = Vec::with_capacity(1);
	for (value, _) in counter_style_data().iter() {
		let input = std::format!("counter(bla, {})", value);
		data.push((input.to_string(), input.to_string()));
	}
	for (value, _) in counter_style_data().iter() {
		let input = std::format!("counters(something, \"hello\", {})", value);
		data.push((input.to_string(), input.to_string()));
	}
	data
}

pub fn target_data() -> Vec<(String, String)> {
	let mut data = Vec::with_capacity(1);
	for ((style, _), value) in permutate(
		counter_style_data().iter(),
		["\"hello\"", "url(\"http://www.example.com\")"].iter(),
	)
	.iter()
	{
		let input = std::format!("target-counter({}, blabla, {})", value, style);
		data.push((input.to_string(), input.to_string()));
	}
	for ((style, _), value) in permutate(
		counter_style_data().iter(),
		["\"hello\"", "url(\"http://www.example.com\")"].iter(),
	)
	.iter()
	{
		let input = std::format!("target-counters({}, blabla, \"utopia\", {})", value, style);
		data.push((input.to_string(), input.to_string()));
	}
	for (link, keyword) in permutate(
		["\"hello\"", "url(\"http://www.example.com\")"].iter(),
		["content", "before", "after", "first-letter", ""].iter(),
	)
	.iter()
	{
		let input = std::format!(
			"target-text({}{})",
			link,
			if keyword.len() > 0 {
				std::format!(", {}", keyword)
			} else {
				"".to_string()
			}
		);
		data.push((input.to_string(), input.to_string()));
	}
	data
}

pub fn leader_data() -> Vec<(String, String)> {
	let mut data = Vec::with_capacity(1);
	for value in ["dotted", "solid", "space", "\"something\""].iter() {
		let input = std::format!("leader({})", value);
		data.push((input.to_string(), input.to_string()));
	}
	data
}
