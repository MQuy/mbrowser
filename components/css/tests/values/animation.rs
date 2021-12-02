pub fn keyframe_name_data() -> Vec<(String, String)> {
	let mut data = Vec::with_capacity(1);
	for name in ["nono79", "ground-level", "-test", "_internal", "chào", "\"hello\""].iter() {
		data.push((name.to_string(), name.to_string()))
	}
	data
}
