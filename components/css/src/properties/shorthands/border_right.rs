use cssparser::Parser;

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::properties::declaration_block::SourcePropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::specified::color::Color;
use crate::values::specified::layout::LineStyle;
use crate::values::specified::line::LineWidth;

pub struct Longhands {
	pub border_right_color: Color,
	pub border_right_style: LineStyle,
	pub border_right_width: LineWidth,
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
			declarations.push(PropertyDeclaration::BorderRightColor(longhands.border_right_color));
			declarations.push(PropertyDeclaration::BorderRightStyle(longhands.border_right_style));
			declarations.push(PropertyDeclaration::BorderRightWidth(longhands.border_right_width));
		})
}
