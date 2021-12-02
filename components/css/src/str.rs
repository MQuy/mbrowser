/// Returns true if a given string has a given prefix with case-insensitive match.
pub fn starts_with_ignore_ascii_case(string: &str, prefix: &str) -> bool {
	string.len() >= prefix.len() && string.as_bytes()[0..prefix.len()].eq_ignore_ascii_case(prefix.as_bytes())
}

pub fn convert_options_to_string(options: Vec<Option<String>>, delimitor: &str) -> String {
	options
		.iter()
		.filter_map(|v| v.as_ref())
		.map(|v| v.to_owned())
		.collect::<Vec<String>>()
		.join(delimitor)
}

pub fn join_strings(values: Vec<&str>, delimitor: &str) -> String {
	values
		.iter()
		.filter(|v| v.len() > 0)
		.map(|v| v.to_string())
		.collect::<Vec<String>>()
		.join(delimitor)
}
