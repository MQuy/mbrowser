use cssparser::Parser;

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::properties::declaration_block::SourcePropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::specified::color::Color;

pub struct Longhands {
	pub border_block_start_color: Color,
	pub border_block_end_color: Color,
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
			declarations.push(PropertyDeclaration::BorderBlockStartColor(
				longhands.border_block_start_color,
			));
			declarations.push(PropertyDeclaration::BorderBlockEndColor(
				longhands.border_block_end_color,
			));
		})
}
