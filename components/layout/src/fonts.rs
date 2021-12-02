use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::sync::Arc;

use font_kit::family_name::FamilyName;
use font_kit::properties::Properties;
use font_kit::source::SystemSource;
use once_cell::sync::Lazy;

use crate::text::FALLBACK;

pub fn load_cached_font(family_names: &Vec<String>) -> (&'static str, &'static [u8]) {
	static mut FONT_CACHED: Lazy<HashMap<String, Arc<Vec<u8>>>> = Lazy::new(|| HashMap::new());

	unsafe {
		let fonts = &mut FONT_CACHED;
		let mut matched_font_name = None;
		for family_name in family_names {
			if let Some(_) = fonts.get(family_name) {
				matched_font_name = Some(family_name);
			} else {
				let font_family = match family_name.as_str() {
					"serif" => FamilyName::Serif,
					"sans-serif" => FamilyName::SansSerif,
					"monospace" => FamilyName::Monospace,
					"cursive" => FamilyName::Cursive,
					"fantasy" => FamilyName::Fantasy,
					_ => FamilyName::Title(family_name.to_string()),
				};
				let system_source = SystemSource::new();
				let handle = system_source.select_best_match(&[font_family], &Properties::default());
				match handle {
					Ok(value) => {
						let bytes = match value {
							font_kit::handle::Handle::Path { path, .. } => {
								let mut buf = Vec::new();
								let mut reader = File::open(path).expect("Read font");
								let _ = reader.read_to_end(&mut buf);
								buf
							},
							font_kit::handle::Handle::Memory { bytes, .. } => bytes.as_ref().clone(),
						};
						fonts.insert(family_name.to_string(), Arc::new(bytes));
						matched_font_name = Some(family_name);
					},
					Err(_) => (),
				};
			}
		}
		if let Some(font_name) = matched_font_name {
			for key in fonts.keys() {
				if key == font_name {
					return (key, fonts.get(font_name).unwrap());
				}
			}
			("fallback", FALLBACK)
		} else {
			("fallback", FALLBACK)
		}
	}
}
