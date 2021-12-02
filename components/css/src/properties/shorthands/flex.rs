use cssparser::Parser;

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::properties::declaration_block::SourcePropertyDeclaration;
use crate::properties::longhands::flex_basis::FlexBasis;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::specified::number::NonNegativeNumber;

pub struct Longhands {
	pub flex_grow: NonNegativeNumber,
	pub flex_shrink: NonNegativeNumber,
	pub flex_basis: FlexBasis,
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
			declarations.push(PropertyDeclaration::FlexGrow(longhands.flex_grow));
			declarations.push(PropertyDeclaration::FlexShrink(longhands.flex_shrink));
			declarations.push(PropertyDeclaration::FlexBasis(longhands.flex_basis));
		})
}
