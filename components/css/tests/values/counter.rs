pub fn counter_with_integer_and_none_data() -> Vec<(String, String)> {
	let mut data = Vec::with_capacity(1);
	for value in [
		"none",
		"something",
		"bla 0",
		"lalala -10 utopia",
		"ping pong ping pong",
	]
	.iter()
	{
		data.push((value.to_string(), value.to_string()))
	}
	data
}

pub fn reversed_counter_with_integer_and_none_data() -> Vec<(String, String)> {
	let mut data = Vec::with_capacity(1);
	for value in [
		"none",
		"reversed(something)",
		"reversed(bla) 0",
		"reversed(lalala) -10 reversed(utopia)",
		"reversed(ping) reversed(pong) reversed(ping) reversed(pong)",
	]
	.iter()
	{
		data.push((value.to_string(), value.to_string()))
	}
	data
}
