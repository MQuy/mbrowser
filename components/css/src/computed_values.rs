use std::collections::HashMap;

use crate::properties::declaration::PropertyDeclaration;
use crate::properties::longhand_id::LonghandId;
use crate::properties::longhands::display::Display;
use crate::values::color::RGBA;
use crate::values::length::LengthPercentageOrAuto;

#[derive(Debug)]
pub struct Margin {
	pub margin_top: LengthPercentageOrAuto,
	pub margin_right: LengthPercentageOrAuto,
	pub margin_bottom: LengthPercentageOrAuto,
	pub margin_left: LengthPercentageOrAuto,
}

#[derive(Debug)]
pub struct Padding {
	pub margin_top: LengthPercentageOrAuto,
	pub margin_right: LengthPercentageOrAuto,
	pub margin_bottom: LengthPercentageOrAuto,
	pub margin_left: LengthPercentageOrAuto,
}

#[derive(Debug)]
pub struct Box {
	pub display: Display,
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
	margin: Margin,
	padding: Padding,
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

	pub fn get_margin_top(&self) -> &LengthPercentageOrAuto {
		&self.margin.margin_top
	}

	pub fn set_margin_top(&mut self, value: LengthPercentageOrAuto) {
		self.margin.margin_top = value;
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
