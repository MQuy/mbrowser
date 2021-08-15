use cssparser::{Delimiter, Parser};

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::color::Color;
use crate::values::length::{Length, NonNegativeLength};

#[derive(Clone)]
pub struct BoxShadowValue {
    inset: bool,
    length: (Length, Length, Option<NonNegativeLength>, Option<Length>),
    color: Option<Color>,
}

impl BoxShadowValue {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        let mut color = None;
        let mut inset = false;
        let mut length = None;

        input.parse_until_before(
            Delimiter::Semicolon,
            |input| -> Result<(), ParseError<'i>> {
                if !inset {
                    inset = input
                        .try_parse(|input| input.expect_ident_matching("inset"))
                        .is_ok();
                }

                if length.is_none() {
                    length = input
                        .try_parse(
                            |input| -> Result<
                                (Length, Length, Option<NonNegativeLength>, Option<Length>),
                                ParseError<'i>,
                            > {
                                let horizontal = Length::parse(context, input)?;
                                let vertical = Length::parse(context, input)?;
                                let blur = input
                                    .try_parse(|input| NonNegativeLength::parse(context, input))
                                    .ok();
                                let spread =
                                    input.try_parse(|input| Length::parse(context, input)).ok();
                                Ok((horizontal, vertical, blur, spread))
                            },
                        )
                        .ok();
                }

                if color.is_none() {
                    color = input.try_parse(|input| Color::parse(context, input)).ok();
                };

                Ok(())
            },
        );

        if let Some(length) = length {
            Ok(BoxShadowValue {
                inset,
                color,
                length,
            })
        } else {
            Err(input.new_custom_error(StyleParseErrorKind::UnspecifiedError))
        }
    }
}

#[derive(Clone)]
pub enum BoxShadow {
    None,
    Shadow(BoxShadowValue),
}

impl BoxShadow {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        input
            .try_parse(|input| {
                input.expect_ident_matching("none")?;
                Ok(BoxShadow::None)
            })
            .or_else(|_err: ParseError<'i>| {
                let value = BoxShadowValue::parse(context, input)?;
                Ok(BoxShadow::Shadow(value))
            })
    }
}

pub fn parse_declared<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
    BoxShadow::parse(context, input).map(PropertyDeclaration::BoxShadow)
}
