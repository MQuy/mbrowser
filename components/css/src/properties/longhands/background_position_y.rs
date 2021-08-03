use cssparser::Parser;

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::length::LengthPercentage;

#[derive(Clone)]
pub enum VerticalPositionKeyword {
    Left,
    Right,
    YStart,
    YEnd,
}

#[derive(Clone)]
pub struct VerticalPosition {
    keyword: Option<VerticalPositionKeyword>,
    length: Option<LengthPercentage>,
}

#[derive(Clone)]
pub enum VerticalPositionComponent {
    Center,
    PositionX,
}

#[derive(Clone)]
pub struct BackgroundPositionY {
    positions: Vec<VerticalPositionComponent>,
}

pub fn parse<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<BackgroundPositionY, ParseError<'i>> {
    panic!()
}

pub fn parse_declared<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
    parse(context, input).map(PropertyDeclaration::BackgroundPositionY)
}
