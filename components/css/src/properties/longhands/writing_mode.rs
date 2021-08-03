use cssparser::Parser;

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;

#[derive(Clone)]
pub enum WritingMode {
    HorizontalTb,
    VerticalRl,
    VerticalLr,
}

pub fn parse<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<WritingMode, ParseError<'i>> {
    panic!()
}

pub fn parse_declared<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
    parse(context, input).map(PropertyDeclaration::WritingMode)
}
