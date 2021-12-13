use cssparser::Parser;

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::specified::length::NonNegativeLengthOrNone;

/// https://drafts.csswg.org/css-transforms-2/#perspective-property
pub fn parse_declared<'i, 't>(
	_context: &ParserContext,
	input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
	NonNegativeLengthOrNone::parse(input).map(PropertyDeclaration::Perspective)
}
