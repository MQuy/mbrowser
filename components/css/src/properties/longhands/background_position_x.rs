use cssparser::{match_ignore_ascii_case, Parser, Token, _cssparser_internal_to_lowercase};

use crate::parser::ParseError;
use crate::properties::declaration::{property_keywords_impl, PropertyDeclaration};
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::length::LengthPercentage;

#[derive(Clone)]
pub enum HorizontalPositionKeyword {
    Left,
    Right,
    XStart,
    XEnd,
}

property_keywords_impl! { HorizontalPositionKeyword,
    HorizontalPositionKeyword::Left, "left",
    HorizontalPositionKeyword::Right, "right",
    HorizontalPositionKeyword::XStart, "x-start",
    HorizontalPositionKeyword::XEnd, "x-end",
}

#[derive(Clone)]
pub struct HorizontalPosition {
    keyword: Option<HorizontalPositionKeyword>,
    length: Option<LengthPercentage>,
}

impl HorizontalPosition {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        let keyword_parser_ret = input.try_parse(|input| HorizontalPositionKeyword::parse(input));
        let length_parser_ret = input.try_parse(|input| LengthPercentage::parse(context, input));

        if keyword_parser_ret.is_err() && length_parser_ret.is_err() {
            Err(input.new_custom_error(StyleParseErrorKind::UnspecifiedError))
        } else {
            Ok(HorizontalPosition {
                keyword: keyword_parser_ret.ok(),
                length: length_parser_ret.ok(),
            })
        }
    }
}

#[derive(Clone)]
pub enum HorizontalPositionComponent {
    Center,
    PositionX(HorizontalPosition),
}

impl HorizontalPositionComponent {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        input
            .try_parse(|input| {
                input.expect_ident_matching("center")?;
                Ok(HorizontalPositionComponent::Center)
            })
            .or_else(|_err: ParseError<'i>| {
                let position = HorizontalPosition::parse(context, input)?;
                Ok(HorizontalPositionComponent::PositionX(position))
            })
    }
}

#[derive(Clone)]
pub struct BackgroundPositionX {
    positions: Vec<HorizontalPositionComponent>,
}

impl BackgroundPositionX {
    /// https://drafts.csswg.org/css-backgrounds-4/#propdef-background-position-x
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        let positions = input
            .parse_comma_separated(|input| HorizontalPositionComponent::parse(context, input))?;
        Ok(BackgroundPositionX { positions })
    }
}

pub fn parse_declared<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
    BackgroundPositionX::parse(context, input).map(PropertyDeclaration::BackgroundPositionX)
}
