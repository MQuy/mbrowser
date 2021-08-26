use cssparser::Parser;

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::number::NonNegativeNumber;

/// https://drafts.csswg.org/css-flexbox/#flex-grow-property
pub fn parse_declared<'i, 't>(
	context: &ParserContext,
	input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
	NonNegativeNumber::parse(context, input).map(PropertyDeclaration::FlexGrow)
}
