use cssparser::Parser;

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::specified::length::LengthPercentageOrAuto;

/// https://drafts.csswg.org/css-logical/#propdef-inset-inline-start
pub fn parse_declared<'i, 't>(
	_context: &ParserContext,
	input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
	LengthPercentageOrAuto::parse(input).map(PropertyDeclaration::InsetInlineStart)
}
