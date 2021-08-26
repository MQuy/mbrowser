use cssparser::Parser;

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::specified::counter::CounterWithInteger;

/// https://drafts.csswg.org/css-lists/#propdef-counter-increment
pub fn parse_declared<'i, 't>(
	context: &ParserContext,
	input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
	CounterWithInteger::parse(context, input).map(PropertyDeclaration::CounterIncrement)
}
