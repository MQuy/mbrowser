use std::collections::HashMap;

use crate::properties::declaration::PropertyDeclaration;
use crate::properties::longhand_id::LonghandId;
use crate::properties::longhands;
use crate::properties::longhands::display::Display;
use crate::properties::longhands::font_size::DEFAULT_FONT_SIZE;
use crate::values::computed::length::{LengthPercentageOrAuto, MaxSize, NonNegativeLengthPercentage, Size};
use crate::values::computed::line::LineWidth;
use crate::values::specified::color::RGBA;
use crate::values::specified::layout::LineStyle;
use crate::values::CSSFloat;

#[derive(Debug)]
pub struct Box {
	pub display: Display,
	pub width: Size,
	pub min_width: Size,
	pub max_width: MaxSize,
	pub height: Size,
	pub min_height: Size,
	pub max_height: MaxSize,
}

impl Default for Box {
	fn default() -> Self {
		Self {
			display: longhands::display::initial_value(),
			width: Size::Auto,
			min_width: Size::Auto,
			max_width: MaxSize::None,
			height: Size::Auto,
			min_height: Size::Auto,
			max_height: MaxSize::None,
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
pub struct BorderEdge {
	pub color: RGBA,
	pub width: LineWidth,
	pub style: LineStyle,
}

impl Default for BorderEdge {
	fn default() -> Self {
		Self {
			color: RGBA::transparent(),
			width: LineWidth::Medium,
			style: LineStyle::None,
		}
	}
}

#[derive(Debug)]
pub struct Border {
	pub border_top: BorderEdge,
	pub border_right: BorderEdge,
	pub border_bottom: BorderEdge,
	pub border_left: BorderEdge,
}

impl Default for Border {
	fn default() -> Self {
		Self {
			border_top: Default::default(),
			border_right: Default::default(),
			border_bottom: Default::default(),
			border_left: Default::default(),
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
	border: Border,
}

impl Default for ComputedValues {
	fn default() -> Self {
		Self {
			background: Default::default(),
			box_: Default::default(),
			text: Default::default(),
			margin: Default::default(),
			padding: Default::default(),
			border: Default::default(),
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

	pub fn get_max_width(&self) -> &MaxSize {
		&self.box_.max_width
	}

	pub fn set_max_width(&mut self, value: MaxSize) {
		self.box_.max_width = value;
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

	pub fn get_max_height(&self) -> &MaxSize {
		&self.box_.max_height
	}

	pub fn set_max_height(&mut self, value: MaxSize) {
		self.box_.max_height = value;
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

	pub fn get_border_top_color(&self) -> &RGBA {
		&self.border.border_top.color
	}

	pub fn set_border_top_color(&mut self, value: RGBA) {
		self.border.border_top.color = value;
	}

	pub fn get_border_top_style(&self) -> &LineStyle {
		&self.border.border_top.style
	}

	pub fn set_border_top_style(&mut self, value: LineStyle) {
		self.border.border_top.style = value;
	}

	pub fn get_border_top_width(&self) -> &LineWidth {
		&self.border.border_top.width
	}

	pub fn set_border_top_width(&mut self, value: LineWidth) {
		self.border.border_top.width = value;
	}

	pub fn get_border_right_color(&self) -> &RGBA {
		&self.border.border_right.color
	}

	pub fn set_border_right_color(&mut self, value: RGBA) {
		self.border.border_right.color = value;
	}

	pub fn get_border_right_style(&self) -> &LineStyle {
		&self.border.border_right.style
	}

	pub fn set_border_right_style(&mut self, value: LineStyle) {
		self.border.border_right.style = value;
	}

	pub fn get_border_right_width(&self) -> &LineWidth {
		&self.border.border_right.width
	}

	pub fn set_border_right_width(&mut self, value: LineWidth) {
		self.border.border_right.width = value;
	}

	pub fn get_border_bottom_color(&self) -> &RGBA {
		&self.border.border_bottom.color
	}

	pub fn set_border_bottom_color(&mut self, value: RGBA) {
		self.border.border_bottom.color = value;
	}

	pub fn get_border_bottom_style(&self) -> &LineStyle {
		&self.border.border_bottom.style
	}

	pub fn set_border_bottom_style(&mut self, value: LineStyle) {
		self.border.border_bottom.style = value;
	}

	pub fn get_border_bottom_width(&self) -> &LineWidth {
		&self.border.border_bottom.width
	}

	pub fn set_border_bottom_width(&mut self, value: LineWidth) {
		self.border.border_bottom.width = value;
	}

	pub fn get_border_left_color(&self) -> &RGBA {
		&self.border.border_left.color
	}

	pub fn set_border_left_color(&mut self, value: RGBA) {
		self.border.border_left.color = value;
	}

	pub fn get_border_left_style(&self) -> &LineStyle {
		&self.border.border_left.style
	}

	pub fn set_border_left_style(&mut self, value: LineStyle) {
		self.border.border_left.style = value;
	}

	pub fn get_border_left_width(&self) -> &LineWidth {
		&self.border.border_left.width
	}

	pub fn set_border_left_width(&mut self, value: LineWidth) {
		self.border.border_left.width = value;
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
