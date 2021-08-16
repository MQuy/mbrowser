use cssparser::Parser;

use crate::parser::{parse_item_if_missing, parse_when, ParseError};
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::color::Color;
use crate::values::length::{Length, NonNegativeLength};

#[derive(Clone)]
pub struct BoxShadowValue {
    inset: bool,
    length: (Length, Length, NonNegativeLength, Length),
    color: Option<Color>,
}

impl BoxShadowValue {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        let mut color = None;
        let mut inset = None;
        let mut length = None;

        parse_when(input, &mut |input| {
            let inset_parser_ret = parse_item_if_missing(input, &mut inset, |input| {
                input
                    .expect_ident_matching("inset")
                    .map_err(|_err| input.new_custom_error(StyleParseErrorKind::UnspecifiedError))
            });
            let length_parser_ret = parse_item_if_missing(input, &mut length, |input| {
                let horizontal = Length::parse(context, input)?;
                let vertical = Length::parse(context, input)?;
                let blur = input
                    .try_parse(|input| NonNegativeLength::parse(context, input))
                    .map_or("0".into(), |length| length);
                let spread = input
                    .try_parse(|input| Length::parse(context, input))
                    .map_or("0".into(), |length| length);
                Ok((horizontal, vertical, blur, spread))
            });
            let color_parser_ret =
                parse_item_if_missing(input, &mut color, |input| Color::parse(context, input));

            vec![inset_parser_ret, length_parser_ret, color_parser_ret]
        });

        if let Some(length) = length {
            Ok(BoxShadowValue {
                inset: inset.is_some(),
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
    Shadow(Vec<BoxShadowValue>),
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
                let shadows = input.parse_comma_separated(|input| {
                    let value = BoxShadowValue::parse(context, input)?;
                    Ok(value)
                })?;
                Ok(BoxShadow::Shadow(shadows))
            })
    }
}

pub fn parse_declared<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
    BoxShadow::parse(context, input).map(PropertyDeclaration::BoxShadow)
}
