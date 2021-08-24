use cssparser::{match_ignore_ascii_case, Parser, ToCss, Token, _cssparser_internal_to_lowercase};

use crate::parser::{parse_in_any_order, parse_item_if_missing, ParseError};
use crate::properties::declaration::property_keywords_impl;
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::length::LengthPercentage;

macro_rules! return_unexpected_token {
    ($location:tt, $token:tt) => {
        return Err($location.new_custom_error(StyleParseErrorKind::UnexpectedToken($token.clone())))
    };
}

#[derive(Clone)]
pub enum LeftOrRight {
    Left,
    Right,
}

property_keywords_impl! { LeftOrRight,
    LeftOrRight::Left, "left",
    LeftOrRight::Right, "right",
}

#[derive(Clone)]
pub enum HorizontalPosition {
    Left,
    Right,
    Center,
    Length(LengthPercentage),
    Side(LeftOrRight, Option<LengthPercentage>),
}

impl HorizontalPosition {
    pub fn parse<'i, 't>(
        _context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        let location = input.current_source_location();
        let token = input.next()?;
        Ok(match token {
            Token::Ident(value) => match_ignore_ascii_case! { value,
                "left" => HorizontalPosition::Left,
                "right" => HorizontalPosition::Right,
                "center" => HorizontalPosition::Center,
                _ => return_unexpected_token!(location, token),
            },
            _ => return_unexpected_token!(location, token),
        })
    }

    pub fn parse_with_length<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        input
            .try_parse(|input| HorizontalPosition::parse(context, input))
            .or_else(|_err: ParseError<'i>| {
                let length = LengthPercentage::parse(context, input)?;
                Ok(HorizontalPosition::Length(length))
            })
    }

    pub fn parse_side<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        input.try_parse(|input| {
            let keyword = LeftOrRight::parse(input)?;
            let length = LengthPercentage::parse(context, input).ok();
            Ok(HorizontalPosition::Side(keyword, length))
        })
    }
}

#[derive(Clone)]
pub enum TopOrBottom {
    Top,
    Bottom,
}

property_keywords_impl! { TopOrBottom,
    TopOrBottom::Top, "top",
    TopOrBottom::Bottom, "bottom",
}

#[derive(Clone)]
pub enum VerticalPosition {
    Top,
    Bottom,
    Center,
    Length(LengthPercentage),
    Side(TopOrBottom, Option<LengthPercentage>),
}

impl VerticalPosition {
    pub fn parse<'i, 't>(
        _context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        let location = input.current_source_location();
        let token = input.next()?;
        Ok(match token {
            Token::Ident(value) => match_ignore_ascii_case! { value,
                "top" => VerticalPosition::Top,
                "bottom" => VerticalPosition::Bottom,
                "center" => VerticalPosition::Center,
                _ => return_unexpected_token!(location, token),
            },
            _ => return_unexpected_token!(location, token),
        })
    }

    pub fn parse_with_length<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        input
            .try_parse(|input| VerticalPosition::parse(context, input))
            .or_else(|_err: ParseError<'i>| {
                let length = LengthPercentage::parse(context, input)?;
                Ok(VerticalPosition::Length(length))
            })
    }

    pub fn parse_side<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        input.try_parse(|input| {
            let keyword = TopOrBottom::parse(input)?;
            let length = LengthPercentage::parse(context, input).ok();
            Ok(VerticalPosition::Side(keyword, length))
        })
    }
}

#[derive(Clone)]
pub struct Position {
    horizontal: HorizontalPosition,
    vertical: VerticalPosition,
}

impl Position {
    pub fn new(horizontal: HorizontalPosition, vertical: VerticalPosition) -> Self {
        Self {
            horizontal,
            vertical,
        }
    }

    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        input
            .try_parse(|input| {
                let mut horizontal = None;
                let mut vertical = None;
                parse_in_any_order(
                    input,
                    &mut [
                        &mut |input| {
                            parse_item_if_missing(input, &mut horizontal, |_, input| {
                                HorizontalPosition::parse(context, input)
                            })
                        },
                        &mut |input| {
                            parse_item_if_missing(input, &mut vertical, |_, input| {
                                VerticalPosition::parse(context, input)
                            })
                        },
                    ],
                );
                if horizontal.is_none() && vertical.is_none() {
                    Err(input.new_custom_error(StyleParseErrorKind::UnspecifiedError))
                } else {
                    Ok(Position {
                        horizontal: horizontal.map_or(HorizontalPosition::Center, |v| v),
                        vertical: vertical.map_or(VerticalPosition::Center, |v| v),
                    })
                }
            })
            .or_else(|_err: ParseError<'i>| {
                input.try_parse(|input| {
                    let horizontal = HorizontalPosition::parse_with_length(context, input)?;
                    let vertical = VerticalPosition::parse_with_length(context, input)?;
                    Ok(Position {
                        horizontal,
                        vertical,
                    })
                })
            })
            .or_else(|_err: ParseError<'i>| {
                let horizontal = HorizontalPosition::parse_side(context, input)?;
                let vertical = VerticalPosition::parse_side(context, input)?;
                Ok(Position {
                    horizontal,
                    vertical,
                })
            })
    }
}
