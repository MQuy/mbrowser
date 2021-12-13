use cssparser::Parser;

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::properties::declaration_block::SourcePropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::specified::length::LengthPercentageOrAuto;

pub struct Longhands {
	pub margin_block_start: LengthPercentageOrAuto,
	pub margin_block_end: LengthPercentageOrAuto,
}

pub fn parse_value<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Longhands, ParseError<'i>> {
	let first = LengthPercentageOrAuto::parse(input)?;
	if let Ok(second) = LengthPercentageOrAuto::parse(input) {
		Ok(Longhands {
			margin_block_start: first,
			margin_block_end: second,
		})
	} else {
		Ok(Longhands {
			margin_block_start: first.clone(),
			margin_block_end: first,
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
		declarations.push(PropertyDeclaration::MarginBlockStart(longhands.margin_block_start));
		declarations.push(PropertyDeclaration::MarginBlockEnd(longhands.margin_block_end));
	})
}
