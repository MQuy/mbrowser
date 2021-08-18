use cssparser::{match_ignore_ascii_case, Parser, _cssparser_internal_to_lowercase};

use crate::parser::{parse_in_any_order, parse_item_if_missing, ParseError};
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::length::LengthPercentage;

#[derive(Clone, PartialEq)]
pub enum OffsetKeyword {
    Left,
    Center,
    Right,
    Top,
    Bottom,
}

impl OffsetKeyword {
    pub fn parse<'i, 't>(
        _context: &ParserContext,
        input: &mut Parser<'i, 't>,
        allowed_keywords: &[OffsetKeyword],
    ) -> Result<Self, ParseError<'i>> {
        let location = input.current_source_location();
        let ident = input.expect_ident()?;
        let keyword = match_ignore_ascii_case! { ident,
            "left" => OffsetKeyword::Left,
            "center" => OffsetKeyword::Center,
            "right" => OffsetKeyword::Right,
            "top" => OffsetKeyword::Top,
            "bottom" => OffsetKeyword::Bottom,
            _ => return Err(location.new_custom_error(StyleParseErrorKind::UnexpectedValue(ident.clone())))
        };
        if allowed_keywords.contains(&keyword) {
            Ok(keyword)
        } else {
            Err(input.new_custom_error(StyleParseErrorKind::UnspecifiedError))
        }
    }
}

#[derive(Clone)]
pub enum LengthPercentageOrKeyword {
    LengthPercentage(LengthPercentage),
    Keyword(OffsetKeyword),
}

#[derive(Clone)]
pub struct TransformOrigin {
    x: LengthPercentageOrKeyword,
    y: LengthPercentageOrKeyword,
    z: LengthPercentageOrKeyword,
}

impl TransformOrigin {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        input
            .try_parse(|input| -> Result<Self, ParseError<'i>> {
                let mut x = None;
                let mut y = None;
                parse_in_any_order(
                    input,
                    &mut [
                        &mut |input| {
                            parse_item_if_missing(input, &mut x, |_, input| {
                                OffsetKeyword::parse(
                                    context,
                                    input,
                                    &[
                                        OffsetKeyword::Center,
                                        OffsetKeyword::Left,
                                        OffsetKeyword::Right,
                                    ],
                                )
                            })
                        },
                        &mut |input| {
                            parse_item_if_missing(input, &mut y, |_, input| {
                                OffsetKeyword::parse(
                                    context,
                                    input,
                                    &[
                                        OffsetKeyword::Center,
                                        OffsetKeyword::Top,
                                        OffsetKeyword::Bottom,
                                    ],
                                )
                            })
                        },
                    ],
                );
                if let (Some(x), Some(y)) = (x, y) {
                    let z = LengthPercentage::parse(context, input).map_or("0px".into(), |v| v);
                    Ok(TransformOrigin {
                        x: LengthPercentageOrKeyword::Keyword(x),
                        y: LengthPercentageOrKeyword::Keyword(y),
                        z: LengthPercentageOrKeyword::LengthPercentage(z),
                    })
                } else {
                    Err(input.new_custom_error(StyleParseErrorKind::UnspecifiedError))
                }
            })
            .or_else(|_err: ParseError<'i>| {
                let x = input
                    .try_parse(
                        |input| {
                            let keyword = OffsetKeyword::parse(
                                context,
                                input,
                                &[
                                    OffsetKeyword::Left,
                                    OffsetKeyword::Center,
                                    OffsetKeyword::Right,
                                ],
                            )?;
                            Ok(LengthPercentageOrKeyword::Keyword(keyword))
                        },
                    )
                    .or_else(|_err: ParseError<'i>| -> Result<LengthPercentageOrKeyword, ParseError<'i>> {
                        let value = input.try_parse(|input| LengthPercentage::parse(context, input))?;
                        Ok(LengthPercentageOrKeyword::LengthPercentage(value))
                    })?;
                let y = input
                    .try_parse(
                        |input| {
                            let keyword = OffsetKeyword::parse(
                                context,
                                input,
                                &[
                                    OffsetKeyword::Top,
                                    OffsetKeyword::Center,
                                    OffsetKeyword::Bottom,
                                ],
                            )?;
                            Ok(LengthPercentageOrKeyword::Keyword(keyword))
                        },
                    )
                    .or_else(|_err: ParseError<'i>| -> Result<LengthPercentageOrKeyword, ParseError<'i>> {
                        let value = input.try_parse(|input| LengthPercentage::parse(context, input))?;
                        Ok(LengthPercentageOrKeyword::LengthPercentage(value))
                    })?;
                let z = input.try_parse(|input| LengthPercentage::parse(context, input)).map_or("0px".into(), |v| v);
                Ok(TransformOrigin {x, y, z: LengthPercentageOrKeyword::LengthPercentage(z)})
            })
            .or_else(|_err: ParseError<'i>| {
                let x = input.try_parse(|input| {
                    let keyword = OffsetKeyword::parse(context, input, &[OffsetKeyword::Left, OffsetKeyword::Center, OffsetKeyword::Right, OffsetKeyword::Top, OffsetKeyword::Bottom])?;
                    Ok(LengthPercentageOrKeyword::Keyword(keyword))
                }).or_else(|_err: ParseError<'i>| -> Result<LengthPercentageOrKeyword, ParseError<'i>> {
                    let value = input.try_parse(|input| LengthPercentage::parse(context, input))?;
                    Ok(LengthPercentageOrKeyword::LengthPercentage(value))
                })?;
                Ok(TransformOrigin {x, y: LengthPercentageOrKeyword::Keyword(OffsetKeyword::Center), z: LengthPercentageOrKeyword::LengthPercentage("0px".into())})
            })
    }
}

pub fn parse_declared<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
    TransformOrigin::parse(context, input).map(PropertyDeclaration::TransformOrigin)
}
