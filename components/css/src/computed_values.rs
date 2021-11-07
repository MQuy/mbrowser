use std::collections::HashMap;

use crate::properties::declaration::PropertyDeclaration;
use crate::properties::longhand_id::LonghandId;
use crate::properties::longhands;
use crate::properties::longhands::display::Display;
use crate::properties::longhands::font_size::DEFAULT_FONT_SIZE;
use crate::values::computed::length::{
	LengthPercentage, LengthPercentageOrAuto, NonNegativeLengthPercentage, Size,
};
use crate::values::specified::color::RGBA;
use crate::values::CSSFloat;

#[derive(Debug)]
pub struct Box {
	pub display: Display,
	pub width: Size,
	pub min_width: Size,
	pub height: Size,
	pub min_height: Size,
}

impl Default for Box {
	fn default() -> Self {
		Self {
			display: longhands::display::initial_value(),
			height: Size::Auto,
			min_height: Size::Auto,
			min_width: Size::Auto,
			width: Size::Auto,
		}
	}
}

#[derive(Debug)]
pub struct Margin {
	pub margin_top: LengthPercentageOrAuto,
	pub margin_right: LengthPercentageOrAuto,
	pub margin_bottom: LengthPercentageOrAuto,
	pub margin_left: LengthPercentageOrAuto,
}

impl Default for Margin {
	fn default() -> Self {
		Self {
			margin_bottom: LengthPercentageOrAuto::zero(),
			margin_left: LengthPercentageOrAuto::zero(),
			margin_right: LengthPercentageOrAuto::zero(),
			margin_top: LengthPercentageOrAuto::zero(),
		}
	}
}

#[derive(Debug)]
pub struct Padding {
	pub padding_top: NonNegativeLengthPercentage,
	pub padding_right: NonNegativeLengthPercentage,
	pub padding_bottom: NonNegativeLengthPercentage,
	pub padding_left: NonNegativeLengthPercentage,
}

impl Default for Padding {
	fn default() -> Self {
		Self {
			padding_bottom: NonNegativeLengthPercentage::zero(),
			padding_left: NonNegativeLengthPercentage::zero(),
			padding_right: NonNegativeLengthPercentage::zero(),
			padding_top: NonNegativeLengthPercentage::zero(),
		}
	}
}

#[derive(Debug)]
pub struct Text {
	pub color: RGBA,
	pub font_families: Vec<String>,
	pub font_size: CSSFloat,
}

impl Default for Text {
	fn default() -> Self {
		Self {
			color: RGBA::transparent(),
			font_families: vec![],
			font_size: DEFAULT_FONT_SIZE,
		}
	}
}

#[derive(Debug)]
pub struct Background {
	pub background_color: RGBA,
}

impl Default for Background {
	fn default() -> Self {
		Self {
			background_color: RGBA::transparent(),
		}
	}
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
		Self {
			background: Default::default(),
			box_: Default::default(),
			text: Default::default(),
			margin: Default::default(),
			padding: Default::default(),
		}
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

	pub fn get_font_families(&self) -> &Vec<String> {
		&self.text.font_families
	}

	pub fn set_font_families(&mut self, font_families: Vec<String>) {
		self.text.font_families = font_families;
	}

	pub fn get_font_size(&self) -> CSSFloat {
		self.text.font_size
	}

	pub fn set_font_size(&mut self, value: CSSFloat) {
		self.text.font_size = value;
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
