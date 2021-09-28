use cssparser::{Parser, ToCss, _cssparser_internal_to_lowercase, match_ignore_ascii_case};

use crate::parser::{parse_in_any_order, parse_item_if_missing, ParseError};
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::specified::length::LengthPercentage;

#[derive(Clone, Debug, PartialEq)]
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

impl ToCss for OffsetKeyword {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		dest.write_str(match self {
			OffsetKeyword::Left => "left",
			OffsetKeyword::Center => "center",
			OffsetKeyword::Right => "right",
			OffsetKeyword::Top => "top",
			OffsetKeyword::Bottom => "bottom",
		})
	}
}

#[derive(Clone, Debug)]
pub enum LengthPercentageOrKeyword {
	LengthPercentage(LengthPercentage),
	Keyword(OffsetKeyword),
}

impl ToCss for LengthPercentageOrKeyword {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		match self {
			LengthPercentageOrKeyword::LengthPercentage(value) => value.to_css(dest),
			LengthPercentageOrKeyword::Keyword(value) => value.to_css(dest),
		}
	}
}

/// https://drafts.csswg.org/css-transforms-1/#transform-origin-property
#[derive(Clone, Debug)]
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
                input.try_parse(|input| {
                    let (x, y) = input.try_parse(|input| {
                        // "center left" matches x-center and fail since y cannot match with left
                        // it is actually valid cases since x, y can appear in any order
                        input.expect_ident_matching("center")?;
                        let x = OffsetKeyword::parse(
                            context,
                            input,
                            &[
                                OffsetKeyword::Center,
                                OffsetKeyword::Left,
                                OffsetKeyword::Right,
                            ],
                        )?;
                        Ok((x, OffsetKeyword::Center))
                    }).or_else(|_err: ParseError<'i>| {
                        let mut x = None;
                        let mut y = None;
                        parse_in_any_order(
                            input,
                            &mut [
                                &mut |input| {
                                    parse_item_if_missing(input, &mut x, &mut |_, input| {
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
                                    parse_item_if_missing(input, &mut y, &mut |_, input| {
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
                            Ok((x, y))
                        } else {
                            Err(input.new_custom_error(StyleParseErrorKind::UnspecifiedError))
                        }
                    })?;
                    let z = LengthPercentage::parse(context, input).map_or("0px".into(), |v| v);
                    Ok(TransformOrigin {
                        x: LengthPercentageOrKeyword::Keyword(x),
                        y: LengthPercentageOrKeyword::Keyword(y),
                        z: LengthPercentageOrKeyword::LengthPercentage(z),
                    })
                })
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

impl ToCss for TransformOrigin {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		self.x.to_css(dest)?;
		dest.write_char(' ')?;
		self.y.to_css(dest)?;
		dest.write_char(' ')?;
		self.z.to_css(dest)
	}
}

pub fn parse_declared<'i, 't>(
	context: &ParserContext,
	input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
	TransformOrigin::parse(context, input).map(PropertyDeclaration::TransformOrigin)
}
