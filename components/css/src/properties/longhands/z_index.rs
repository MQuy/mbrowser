use cssparser::Parser;

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::number::IntegerAuto;

/// https://drafts.csswg.org/css2/#z-index
pub fn parse_declared<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
    IntegerAuto::parse(context, input).map(PropertyDeclaration::ZIndex)
}
