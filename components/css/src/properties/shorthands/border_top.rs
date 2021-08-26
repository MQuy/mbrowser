use cssparser::Parser;

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::properties::declaration_block::SourcePropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::color::Color;
use crate::values::layout::LineStyle;
use crate::values::specified::line::LineWidth;

pub struct Longhands {
	pub border_top_color: Color,
	pub border_top_style: LineStyle,
	pub border_top_width: LineWidth,
}

pub fn parse_value<'i, 't>(
	context: &ParserContext,
	input: &mut Parser<'i, 't>,
) -> Result<Longhands, ParseError<'i>> {
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
			declarations.push(PropertyDeclaration::BorderTopColor(
				longhands.border_top_color,
			));
			declarations.push(PropertyDeclaration::BorderTopStyle(
				longhands.border_top_style,
			));
			declarations.push(PropertyDeclaration::BorderTopWidth(
				longhands.border_top_width,
			));
		})
}
