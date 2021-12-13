use cssparser::Parser;

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::properties::declaration_block::SourcePropertyDeclaration;
use crate::properties::longhands::border_image_repeat::BorderImageRepeat;
use crate::properties::longhands::border_image_slice::BorderImageSlice;
use crate::properties::longhands::border_image_source::BorderImageSource;
use crate::properties::longhands::border_image_width::BorderImageWidth;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::specified::color::Color;
use crate::values::specified::layout::LineStyle;
use crate::values::specified::length::NonNegativeLengthOrNumberRect;
use crate::values::specified::line::LineWidth;

pub struct Longhands {
	pub border_top_color: Color,
	pub border_top_style: LineStyle,
	pub border_top_width: LineWidth,
	pub border_right_color: Color,
	pub border_right_style: LineStyle,
	pub border_right_width: LineWidth,
	pub border_bottom_color: Color,
	pub border_bottom_style: LineStyle,
	pub border_bottom_width: LineWidth,
	pub border_left_color: Color,
	pub border_left_style: LineStyle,
	pub border_left_width: LineWidth,
	pub border_image_outset: NonNegativeLengthOrNumberRect,
	pub border_image_repeat: BorderImageRepeat,
	pub border_image_slice: BorderImageSlice,
	pub border_image_source: BorderImageSource,
	pub border_image_width: BorderImageWidth,
}

pub fn parse_value<'i, 't>(_context: &ParserContext, _input: &mut Parser<'i, 't>) -> Result<Longhands, ParseError<'i>> {
	todo!()
}

/// Parse the given shorthand and fill the result into the
/// `declarations` vector.
pub fn parse_into<'i, 't>(
	declarations: &mut SourcePropertyDeclaration,
	context: &ParserContext,
	input: &mut Parser<'i, 't>,
) -> Result<(), ParseError<'i>> {
	input
		.parse_entirely(|input| parse_value(context, input))
		.map(|longhands| {
			declarations.push(PropertyDeclaration::BorderTopColor(longhands.border_top_color));
			declarations.push(PropertyDeclaration::BorderTopStyle(longhands.border_top_style));
			declarations.push(PropertyDeclaration::BorderTopWidth(longhands.border_top_width));
			declarations.push(PropertyDeclaration::BorderRightColor(longhands.border_right_color));
			declarations.push(PropertyDeclaration::BorderRightStyle(longhands.border_right_style));
			declarations.push(PropertyDeclaration::BorderRightWidth(longhands.border_right_width));
			declarations.push(PropertyDeclaration::BorderBottomColor(longhands.border_bottom_color));
			declarations.push(PropertyDeclaration::BorderBottomStyle(longhands.border_bottom_style));
			declarations.push(PropertyDeclaration::BorderBottomWidth(longhands.border_bottom_width));
			declarations.push(PropertyDeclaration::BorderLeftColor(longhands.border_left_color));
			declarations.push(PropertyDeclaration::BorderLeftStyle(longhands.border_left_style));
			declarations.push(PropertyDeclaration::BorderLeftWidth(longhands.border_left_width));
			declarations.push(PropertyDeclaration::BorderImageOutset(longhands.border_image_outset));
			declarations.push(PropertyDeclaration::BorderImageRepeat(longhands.border_image_repeat));
			declarations.push(PropertyDeclaration::BorderImageSlice(longhands.border_image_slice));
			declarations.push(PropertyDeclaration::BorderImageSource(longhands.border_image_source));
			declarations.push(PropertyDeclaration::BorderImageWidth(longhands.border_image_width));
		})
}
