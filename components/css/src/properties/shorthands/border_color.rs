use cssparser::Parser;

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::properties::declaration_block::SourcePropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::specified::color::Color;

pub struct Longhands {
	pub border_top_color: Color,
	pub border_right_color: Color,
	pub border_bottom_color: Color,
	pub border_left_color: Color,
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
			declarations.push(PropertyDeclaration::BorderTopColor(longhands.border_top_color));
			declarations.push(PropertyDeclaration::BorderRightColor(longhands.border_right_color));
			declarations.push(PropertyDeclaration::BorderBottomColor(longhands.border_bottom_color));
			declarations.push(PropertyDeclaration::BorderLeftColor(longhands.border_left_color));
		})
}
