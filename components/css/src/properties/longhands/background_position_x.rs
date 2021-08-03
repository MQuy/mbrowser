use cssparser::Parser;

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::length::LengthPercentage;

#[derive(Clone)]
pub enum HorizontalPositionKeyword {
    Left,
    Right,
    XStart,
    XEnd,
}

#[derive(Clone)]
pub struct HorizontalPosition {
    keyword: Option<HorizontalPositionKeyword>,
    length: Option<LengthPercentage>,
}

#[derive(Clone)]
pub enum HorizontalPositionComponent {
    Center,
    PositionX,
}

#[derive(Clone)]
pub struct BackgroundPositionX {
    positions: Vec<HorizontalPositionComponent>,
}

pub fn parse<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<BackgroundPositionX, ParseError<'i>> {
    panic!()
}

pub fn parse_declared<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
    parse(context, input).map(PropertyDeclaration::BackgroundPositionX)
}
