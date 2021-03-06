use cssparser::Parser;

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::specified::length::NonNegativeLengthOrAuto;

/// https://drafts.csswg.org/css-multicol-1/#cw
pub fn parse_declared<'i, 't>(
	_context: &ParserContext,
	input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
	NonNegativeLengthOrAuto::parse(input).map(PropertyDeclaration::ColumnWidth)
}
