use cssparser::Parser;

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::number::Number;

#[derive(Clone)]
pub enum AbsoluteFontWeight {
    Weight(Number),
    Normal,
    Bold,
}

#[derive(Clone)]
pub enum FontWeight {
    Absolute(AbsoluteFontWeight),
    Bolder,
    Lighter,
}

pub fn parse<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<FontWeight, ParseError<'i>> {
    todo!()
}

pub fn parse_declared<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
    parse(context, input).map(PropertyDeclaration::FontWeight)
}
