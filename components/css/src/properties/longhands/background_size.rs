use cssparser::Parser;

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::length::LengthPercentageOrAuto;

#[derive(Clone)]
pub enum BackgroundSize {
    ExplicitSize {
        width: LengthPercentageOrAuto,
        height: LengthPercentageOrAuto,
    },
    Cover,
    Contain,
}

pub fn parse<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<BackgroundSize, ParseError<'i>> {
    todo!()
}

pub fn parse_declared<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
    parse(context, input).map(PropertyDeclaration::BackgroundSize)
}
