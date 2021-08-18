use cssparser::Parser;

use crate::parser::{parse_in_any_order, parse_item_if_missing, ParseError};
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::color::Color;
use crate::values::length::{Length, NonNegativeLength};

#[derive(Clone)]
pub struct SingleTextShadow {
    color: Option<Color>,
    shadow: (Length, Length, NonNegativeLength),
}

impl SingleTextShadow {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        let mut color = None;
        let mut shadow = None;
        parse_in_any_order(
            input,
            &mut [
                &mut |input| {
                    parse_item_if_missing(input, &mut color, |_, input| {
                        Color::parse(context, input)
                    })
                },
                &mut |input| {
                    parse_item_if_missing(input, &mut shadow, |_, input| {
                        let horizontal = Length::parse(context, input)?;
                        let vertical = Length::parse(context, input)?;
                        let blur = NonNegativeLength::parse(context, input)
                            .map_or("0".into(), |value| value);
                        Ok((horizontal, vertical, blur))
                    })
                },
            ],
        );

        if let Some(shadow) = shadow {
            Ok(SingleTextShadow { color, shadow })
        } else {
            Err(input.new_custom_error(StyleParseErrorKind::UnspecifiedError))
        }
    }
}

#[derive(Clone)]
pub struct TextShadow(Vec<SingleTextShadow>);

impl TextShadow {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        let values =
            input.parse_comma_separated(|input| SingleTextShadow::parse(context, input))?;
        Ok(TextShadow(values))
    }
}

pub fn parse_declared<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
    TextShadow::parse(context, input).map(PropertyDeclaration::TextShadow)
}
