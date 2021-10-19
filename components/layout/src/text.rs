use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use font_kit::family_name::FamilyName;
use font_kit::properties::Properties;
use font_kit::source::SystemSource;
use glyph_brush::ab_glyph::{Font, FontArc, GlyphId};
use glyph_brush::{FontId, GlyphCruncher};

pub struct TextUI {
	brush: RefCell<glyph_brush::GlyphBrush<()>>,
	font_map: RefCell<HashMap<String, FontId>>,
}

pub const FALLBACK: &[u8] = include_bytes!("../fonts/Lato-Regular.ttf");

impl TextUI {
	pub fn new() -> Self {
		let font = FontArc::try_from_slice(FALLBACK).expect("default font doesn't exist");
		let brush = glyph_brush::GlyphBrushBuilder::using_font(font).build();
		Self {
			brush: RefCell::new(brush),
			font_map: RefCell::new(Default::default()),
		}
	}

	pub fn measure_size<T: AsRef<str>>(
		&self,
		content: &str,
		family_names: &[T],
		font_size: f32,
	) -> (f32, f32) {
		let groups = self.matching_fonts(content, family_names);
		let section = wgpu_glyph::Section {
			text: groups
				.iter()
				.map(|(text, font_id)| wgpu_glyph::Text {
					text,
					scale: font_size.into(),
					font_id: font_id.clone(),
					extra: wgpu_glyph::Extra::default(),
				})
				.collect(),
			..Default::default()
		};
		if let Some(rect) = self.brush.borrow_mut().glyph_bounds(section) {
			(rect.width(), rect.height())
		} else {
			(0.0, 0.0)
		}
	}

	/*
	- for each character in text, finding a font in font names array (in that order)
	  which (load that font if not loading yet, implementing cache system for font) supports that character
	- construct array (element is [string slice, font])
	  if current font is the same last array, modify last string slice
	 */
	pub fn matching_fonts<T: AsRef<str>>(
		&self,
		content: &str,
		family_names: &[T],
	) -> Vec<(String, FontId)> {
		let mut latest_font = None;
		let mut styles: Vec<(Vec<char>, FontId)> = Vec::with_capacity(1);
		for ch in content.chars() {
			if let Some(ref font) = latest_font {
				if let Some((segment, _)) = styles.last_mut() {
					if self.is_character_supported_by_font(ch, font) {
						segment.push(ch);
						continue;
					}
				}
			}
			let font_id = self.load_font(family_names);
			latest_font = self.get_font_in_brush(font_id);
			styles.push((vec![ch], font_id))
		}
		styles
			.iter()
			.map(|(text, font)| (text.iter().collect::<String>(), font.clone()))
			.collect::<Vec<(String, FontId)>>()
	}

	pub fn load_font<T: AsRef<str>>(&self, family_names: &[T]) -> FontId {
		for family_name in family_names {
			if let Some(font) = self.font_map.borrow().get(family_name.as_ref()) {
				return font.clone();
			} else if let Some(font) = self.load_system_font(family_name.as_ref()) {
				return font;
			}
		}
		FontId(0)
	}

	// TODO: support external font (via font-face)
	pub fn load_external_font(family_name: &str) -> Option<FontArc> {
		todo!()
	}

	pub fn load_system_font(&self, family_name: &str) -> Option<FontId> {
		let font_family = match family_name {
			"serif" => FamilyName::Serif,
			"sans-serif" => FamilyName::SansSerif,
			"monospace" => FamilyName::Monospace,
			"cursive" => FamilyName::Cursive,
			"fantasy" => FamilyName::Fantasy,
			_ => FamilyName::Title(family_name.to_string()),
		};
		let system_source = SystemSource::new();
		let handle = system_source.select_best_match(&[font_family], &Properties::default());
		let bytes = match handle {
			Ok(value) => match value {
				font_kit::handle::Handle::Path { path, .. } => {
					let mut buf = Vec::new();
					let mut reader = File::open(path).expect("Read font");
					let _ = reader.read_to_end(&mut buf);
					buf
				},
				font_kit::handle::Handle::Memory { bytes, .. } => bytes.as_ref().clone(),
			},
			Err(_) => {
				return None;
			},
		};
		if let Ok(font) = FontArc::try_from_vec(bytes) {
			let font_id = self.brush.borrow_mut().add_font(font.clone());
			self.font_map
				.borrow_mut()
				.insert(family_name.to_string(), font_id);
			Some(font_id)
		} else {
			None
		}
	}

	pub fn get_font_in_brush(&self, font_id: FontId) -> Option<FontArc> {
		if let Some(font) = self.brush.borrow().fonts().get(font_id.0) {
			Some(font.clone())
		} else {
			None
		}
	}

	// GlyphId 0 is a special glyph representing a missing character
	pub fn is_character_supported_by_font(&self, ch: char, font: &FontArc) -> bool {
		font.glyph_id(ch) != GlyphId(0)
	}
}
