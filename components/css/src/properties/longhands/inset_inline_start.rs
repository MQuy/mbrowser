use cssparser::Parser;

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::length::LengthPercentageOrAuto;

pub fn parse<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<LengthPercentageOrAuto, ParseError<'i>> {
    todo!()
}

pub fn parse_declared<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
    parse(context, input).map(PropertyDeclaration::InsetInlineStart)
}