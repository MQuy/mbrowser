use std::collections::HashMap;

use crate::properties::declaration::PropertyDeclaration;
use crate::properties::longhand_id::LonghandId;
use crate::properties::longhands::display::Display;
use crate::values::computed::length::{LengthPercentageOrAuto, NonNegativeLengthPercentage, Size};
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
pub struct Margin {
	pub margin_top: LengthPercentageOrAuto,
	pub margin_right: LengthPercentageOrAuto,
	pub margin_bottom: LengthPercentageOrAuto,
	pub margin_left: LengthPercentageOrAuto,
}

#[derive(Debug)]
pub struct Padding {
	pub padding_top: NonNegativeLengthPercentage,
	pub padding_right: NonNegativeLengthPercentage,
	pub padding_bottom: NonNegativeLengthPercentage,
	pub padding_left: NonNegativeLengthPercentage,
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
	margin: Margin,
	padding: Padding,
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

	pub fn get_margin_top(&self) -> &LengthPercentageOrAuto {
		&self.margin.margin_top
	}

	pub fn set_margin_top(&mut self, value: LengthPercentageOrAuto) {
		self.margin.margin_top = value;
	}

	pub fn get_margin_right(&self) -> &LengthPercentageOrAuto {
		&self.margin.margin_right
	}

	pub fn set_margin_right(&mut self, value: LengthPercentageOrAuto) {
		self.margin.margin_right = value;
	}

	pub fn get_margin_bottom(&self) -> &LengthPercentageOrAuto {
		&self.margin.margin_bottom
	}

	pub fn set_margin_bottom(&mut self, value: LengthPercentageOrAuto) {
		self.margin.margin_bottom = value;
	}

	pub fn get_margin_left(&self) -> &LengthPercentageOrAuto {
		&self.margin.margin_left
	}

	pub fn set_margin_left(&mut self, value: LengthPercentageOrAuto) {
		self.margin.margin_left = value;
	}

	pub fn get_padding_top(&self) -> &NonNegativeLengthPercentage {
		&self.padding.padding_top
	}

	pub fn set_padding_top(&mut self, value: NonNegativeLengthPercentage) {
		self.padding.padding_top = value;
	}

	pub fn get_padding_right(&self) -> &NonNegativeLengthPercentage {
		&self.padding.padding_right
	}

	pub fn set_padding_right(&mut self, value: NonNegativeLengthPercentage) {
		self.padding.padding_right = value;
	}

	pub fn get_padding_bottom(&self) -> &NonNegativeLengthPercentage {
		&self.padding.padding_bottom
	}

	pub fn set_padding_bottom(&mut self, value: NonNegativeLengthPercentage) {
		self.padding.padding_bottom = value;
	}

	pub fn get_padding_left(&self) -> &NonNegativeLengthPercentage {
		&self.padding.padding_left
	}

	pub fn set_padding_left(&mut self, value: NonNegativeLengthPercentage) {
		self.padding.padding_left = value;
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
