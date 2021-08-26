/// Returns true if a given string has a given prefix with case-insensitive match.
pub fn starts_with_ignore_ascii_case(string: &str, prefix: &str) -> bool {
	string.len() >= prefix.len()
		&& string.as_bytes()[0..prefix.len()].eq_ignore_ascii_case(prefix.as_bytes())
}
