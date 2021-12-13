use cssparser::Parser;

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::properties::declaration_block::SourcePropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::specified::length::NonNegativeLengthPercentage;

pub struct Longhands {
	pub padding_block_start: NonNegativeLengthPercentage,
	pub padding_block_end: NonNegativeLengthPercentage,
}

pub fn parse_value<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Longhands, ParseError<'i>> {
	let first = NonNegativeLengthPercentage::parse(input)?;
	if let Ok(second) = NonNegativeLengthPercentage::parse(input) {
		Ok(Longhands {
			padding_block_start: first,
			padding_block_end: second,
		})
	} else {
		Ok(Longhands {
			padding_block_start: first.clone(),
			padding_block_end: first,
		})
	}
}

/// Parse the given shorthand and fill the result into the
/// `declarations` vector.
pub fn parse_into<'i, 't>(
	declarations: &mut SourcePropertyDeclaration,
	_context: &ParserContext,
	input: &mut Parser<'i, 't>,
) -> Result<(), ParseError<'i>> {
	input.parse_entirely(|input| parse_value(input)).map(|longhands| {
		declarations.push(PropertyDeclaration::PaddingBlockStart(longhands.padding_block_start));
		declarations.push(PropertyDeclaration::PaddingBlockEnd(longhands.padding_block_end));
	})
}
