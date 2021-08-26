use cssparser::Parser;

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::properties::declaration_block::SourcePropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::length::NonNegativeLengthPercentage;

pub struct Longhands {
	pub padding_top: NonNegativeLengthPercentage,
	pub padding_right: NonNegativeLengthPercentage,
	pub padding_bottom: NonNegativeLengthPercentage,
	pub padding_left: NonNegativeLengthPercentage,
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
			declarations.push(PropertyDeclaration::PaddingTop(longhands.padding_top));
			declarations.push(PropertyDeclaration::PaddingRight(longhands.padding_right));
			declarations.push(PropertyDeclaration::PaddingBottom(longhands.padding_bottom));
			declarations.push(PropertyDeclaration::PaddingLeft(longhands.padding_left));
		})
}
