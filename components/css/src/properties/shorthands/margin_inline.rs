use cssparser::Parser;

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::properties::declaration_block::SourcePropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::specified::length::LengthPercentageOrAuto;

pub struct Longhands {
	pub margin_inline_start: LengthPercentageOrAuto,
	pub margin_inline_end: LengthPercentageOrAuto,
}

pub fn parse_value<'i, 't>(context: &ParserContext, input: &mut Parser<'i, 't>) -> Result<Longhands, ParseError<'i>> {
	let first = LengthPercentageOrAuto::parse(context, input)?;
	if let Ok(second) = LengthPercentageOrAuto::parse(context, input) {
		Ok(Longhands {
			margin_inline_start: first,
			margin_inline_end: second,
		})
	} else {
		Ok(Longhands {
			margin_inline_start: first.clone(),
			margin_inline_end: first,
		})
	}
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
			declarations.push(PropertyDeclaration::MarginInlineStart(longhands.margin_inline_start));
			declarations.push(PropertyDeclaration::MarginInlineEnd(longhands.margin_inline_end));
		})
}
