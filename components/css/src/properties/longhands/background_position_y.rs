use cssparser::{match_ignore_ascii_case, Parser, Token, _cssparser_internal_to_lowercase};

use crate::parser::ParseError;
use crate::properties::declaration::{property_keywords_impl, PropertyDeclaration};
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::length::LengthPercentage;

#[derive(Clone)]
pub enum VerticalPositionKeyword {
    Left,
    Right,
    YStart,
    YEnd,
}

property_keywords_impl! { VerticalPositionKeyword,
    VerticalPositionKeyword::Left, "left",
    VerticalPositionKeyword::Right, "right",
    VerticalPositionKeyword::YStart, "y-start",
    VerticalPositionKeyword::YEnd, "y-end",
}

#[derive(Clone)]
pub struct VerticalPosition {
    keyword: Option<VerticalPositionKeyword>,
    length: Option<LengthPercentage>,
}

impl VerticalPosition {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        todo!()
    }
}

#[derive(Clone)]
pub enum VerticalPositionComponent {
    Center,
    PositionY(VerticalPosition),
}

impl VerticalPositionComponent {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        todo!()
    }
}

#[derive(Clone)]
pub struct BackgroundPositionY {
    positions: Vec<VerticalPositionComponent>,
}

impl BackgroundPositionY {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        let positions = input
            .parse_comma_separated(|input| VerticalPositionComponent::parse(context, input))?;
        Ok(BackgroundPositionY { positions })
    }
}

pub fn parse_declared<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
    BackgroundPositionY::parse(context, input).map(PropertyDeclaration::BackgroundPositionY)
}
