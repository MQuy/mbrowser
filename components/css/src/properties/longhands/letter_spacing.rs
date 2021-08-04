use cssparser::Parser;

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::length::Length;
use crate::values::text::Spacing;

pub type LetterSpacing = Spacing<Length>;

pub fn parse<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<LetterSpacing, ParseError<'i>> {
    todo!()
}

pub fn parse_declared<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
    parse(context, input).map(PropertyDeclaration::LetterSpacing)
}
