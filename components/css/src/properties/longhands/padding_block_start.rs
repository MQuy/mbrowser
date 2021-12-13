use cssparser::Parser;

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::specified::length::NonNegativeLengthPercentage;

/// https://drafts.csswg.org/css-logical/#propdef-padding-block-start
pub fn parse_declared<'i, 't>(
	_context: &ParserContext,
	input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
	NonNegativeLengthPercentage::parse(input).map(PropertyDeclaration::PaddingBlockStart)
}
