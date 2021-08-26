use cssparser::Parser;

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::layout::Overflow;

/// https://drafts.csswg.org/css-overflow/#propdef-overflow-block
pub fn parse_declared<'i, 't>(
	_context: &ParserContext,
	input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
	Overflow::parse(input).map(PropertyDeclaration::OverflowBlock)
}
