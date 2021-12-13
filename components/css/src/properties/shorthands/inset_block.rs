use cssparser::Parser;

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::properties::declaration_block::SourcePropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::specified::length::LengthPercentageOrAuto;

pub struct Longhands {
	pub inset_block_start: LengthPercentageOrAuto,
	pub inset_block_end: LengthPercentageOrAuto,
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
			declarations.push(PropertyDeclaration::InsetBlockStart(longhands.inset_block_start));
			declarations.push(PropertyDeclaration::InsetBlockEnd(longhands.inset_block_end));
		})
}
