use cssparser::Parser;

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::specified::length::NonNegativeLengthPercentageOrNormal;

/// https://drafts.csswg.org/css-align-3/#propdef-column-gap
pub fn parse_declared<'i, 't>(
	_context: &ParserContext,
	input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
	NonNegativeLengthPercentageOrNormal::parse(input).map(PropertyDeclaration::ColumnGap)
}
