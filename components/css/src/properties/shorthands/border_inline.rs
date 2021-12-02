use cssparser::Parser;

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::properties::declaration_block::SourcePropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::specified::color::Color;
use crate::values::specified::layout::LineStyle;
use crate::values::specified::line::LineWidth;

pub struct Longhands {
	pub border_inline_start_width: LineWidth,
	pub border_inline_end_width: LineWidth,
	pub border_inline_start_style: LineStyle,
	pub border_inline_end_style: LineStyle,
	pub border_inline_start_color: Color,
	pub border_inline_end_color: Color,
}

pub fn parse_value<'i, 't>(context: &ParserContext, input: &mut Parser<'i, 't>) -> Result<Longhands, ParseError<'i>> {
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
			declarations.push(PropertyDeclaration::BorderInlineStartWidth(
				longhands.border_inline_start_width,
			));
			declarations.push(PropertyDeclaration::BorderInlineEndWidth(
				longhands.border_inline_end_width,
			));
			declarations.push(PropertyDeclaration::BorderInlineStartStyle(
				longhands.border_inline_start_style,
			));
			declarations.push(PropertyDeclaration::BorderInlineEndStyle(
				longhands.border_inline_end_style,
			));
			declarations.push(PropertyDeclaration::BorderInlineStartColor(
				longhands.border_inline_start_color,
			));
			declarations.push(PropertyDeclaration::BorderInlineEndColor(
				longhands.border_inline_end_color,
			));
		})
}
