use std::collections::HashMap;

use crate::properties::declaration::PropertyDeclaration;
use crate::properties::longhand_id::LonghandId;
use crate::properties::longhands::display::Display;
use crate::values::computed::length::Size;
use crate::values::specified::color::RGBA;

#[derive(Debug)]
pub struct Box {
	pub display: Display,
	pub width: Size,
	pub min_width: Size,
	pub height: Size,
	pub min_height: Size,
}

#[derive(Debug)]
pub struct Text {
	pub color: RGBA,
}

#[derive(Debug)]
pub struct Background {
	pub background_color: RGBA,
}

#[derive(Debug)]
pub struct ComputedValues {
	background: Background,
	box_: Box,
	text: Text,
}

impl Default for ComputedValues {
	fn default() -> Self {
		todo!()
	}
}

impl ComputedValues {
	pub fn get_display(&self) -> &Display {
		&self.box_.display
	}

	pub fn set_display(&mut self, value: Display) {
		self.box_.display = value;
	}

	pub fn get_width(&self) -> &Size {
		&self.box_.width
	}

	pub fn set_width(&mut self, value: Size) {
		self.box_.width = value;
	}

	pub fn get_min_width(&self) -> &Size {
		&self.box_.min_width
	}

	pub fn set_min_width(&mut self, value: Size) {
		self.box_.min_width = value;
	}

	pub fn get_height(&self) -> &Size {
		&self.box_.height
	}

	pub fn set_height(&mut self, value: Size) {
		self.box_.height = value;
	}

	pub fn get_min_height(&self) -> &Size {
		&self.box_.min_height
	}

	pub fn set_min_height(&mut self, value: Size) {
		self.box_.min_height = value;
	}

	pub fn get_color(&self) -> &RGBA {
		&self.text.color
	}

	pub fn set_color(&mut self, value: RGBA) {
		self.text.color = value;
	}

	pub fn get_background_color(&self) -> &RGBA {
		&self.background.background_color
	}

	pub fn set_background_color(&mut self, value: RGBA) {
		self.background.background_color = value;
	}
}

pub struct StyleContext<'a, 'b, 'c, 'd> {
	pub author_data: HashMap<LonghandId, PropertyCascade<'a>>,
	pub useragent_data: HashMap<LonghandId, PropertyCascade<'d>>,
	pub computed_values: &'b mut ComputedValues,
	pub parent_style: &'c ComputedValues,
}

pub struct PropertyCascade<'a> {
	pub specificity: u32,
	pub importance: bool,
	pub property: &'a PropertyDeclaration,
}
