use cssparser::Parser;

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::specified::length::NonNegativeLengthPercentageNumberOrNormal;

/// https://drafts.csswg.org/css-inline/#line-height-property
pub type LineHeight = NonNegativeLengthPercentageNumberOrNormal;

pub fn parse_declared<'i, 't>(
	_context: &ParserContext,
	input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
	LineHeight::parse(input).map(PropertyDeclaration::LineHeight)
}
