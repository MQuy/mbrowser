use euclid::Length;

use crate::computed_values::{Background, Text};
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

#[derive(Debug)]
pub struct Margin {
	pub margin_top: Pixel,
	pub margin_right: Pixel,
	pub margin_bottom: Pixel,
	pub margin_left: Pixel,
}

#[derive(Debug)]
pub struct Padding {
	pub padding_top: Pixel,
	pub padding_right: Pixel,
	pub padding_bottom: Pixel,
	pub padding_left: Pixel,
}

#[derive(Debug)]
pub struct UsedValues {
	background: Background,
	box_: Box,
	text: Text,
	margin: Margin,
	padding: Padding,
}

impl Default for UsedValues {
	fn default() -> Self {
		todo!()
	}
}
