use euclid::Length;

use crate::computed_values::{Background, Text};
use crate::properties::longhands;
use crate::properties::longhands::display::Display;
use crate::values::CSSPixel;

type Pixel = Length<f32, CSSPixel>;

#[derive(Debug)]
pub struct Box {
	pub display: Display,
	pub width: Pixel,
	pub min_width: Pixel,
	pub height: Pixel,
	pub min_height: Pixel,
}

impl Default for Box {
	fn default() -> Self {
		Self {
			display: longhands::display::initial_value(),
			width: Default::default(),
			min_width: Default::default(),
			height: Default::default(),
			min_height: Default::default(),
		}
	}
}

#[derive(Debug)]
pub struct Margin {
	pub margin_top: Pixel,
	pub margin_right: Pixel,
	pub margin_bottom: Pixel,
	pub margin_left: Pixel,
}

impl Default for Margin {
	fn default() -> Self {
		Self {
			margin_top: Default::default(),
			margin_right: Default::default(),
			margin_bottom: Default::default(),
			margin_left: Default::default(),
		}
	}
}

#[derive(Debug)]
pub struct Padding {
	pub padding_top: Pixel,
	pub padding_right: Pixel,
	pub padding_bottom: Pixel,
	pub padding_left: Pixel,
}

impl Default for Padding {
	fn default() -> Self {
		Self {
			padding_top: Default::default(),
			padding_right: Default::default(),
			padding_bottom: Default::default(),
			padding_left: Default::default(),
		}
	}
}

#[derive(Debug)]
pub struct UsedValues {
	background: Background,
	box_: Box,
	text: Text,
	margin: Margin,
	padding: Padding,
}

impl UsedValues {
	pub fn set_width(&mut self, width: f32) {
		self.box_.width = Pixel::new(width);
	}

	pub fn get_width(&self) -> Pixel {
		self.box_.width
	}

	pub fn set_margin_left(&mut self, margin_left: f32) {
		self.margin.margin_left = Pixel::new(margin_left);
	}

	pub fn set_margin_right(&mut self, margin_right: f32) {
		self.margin.margin_right = Pixel::new(margin_right);
	}
}

impl Default for UsedValues {
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
