use cssparser::{match_ignore_ascii_case, Parser, Token, _cssparser_internal_to_lowercase};
use regex::Regex;

use crate::parser::ParseError;
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::percentage::Percentage;
use crate::values::CSSFloat;

#[derive(Clone, PartialEq)]
pub enum Angle {
    Deg(CSSFloat),
    Grad(CSSFloat),
    Rad(CSSFloat),
    Turn(CSSFloat),
}

impl Angle {
    pub fn parse<'i, 't>(
        _context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        macro_rules! return_unexpected_token {
            ($location:tt, $token:tt) => {
                return Err($location
                    .new_custom_error(StyleParseErrorKind::UnexpectedToken($token.clone())))
            };
        }

        let location = input.current_source_location();
        let token = input.next()?;
        Ok(match token {
            Token::Dimension { value, unit, .. } => match_ignore_ascii_case! { unit,
                "deg" => Angle::Deg(*value),
                "grad" => Angle::Grad(*value),
                "rad" => Angle::Rad(*value),
                "turn" => Angle::Turn(*value),
                _ => return_unexpected_token!(location, token)
            },
            _ => return_unexpected_token!(location, token),
        })
    }
}

impl From<&str> for Angle {
    fn from(text: &str) -> Self {
        let regex = Regex::new(r"/deg|grad|rad|turn/i").unwrap();
        let index = regex.find(text).unwrap().start();
        let (value, unit) = (&text[..index], &text[index..]);
        let value = value.parse::<f32>().unwrap();
        match_ignore_ascii_case! { unit,
            "deg" => Angle::Deg(value),
            "grad" => Angle::Grad(value),
            "rad" => Angle::Rad(value),
            "turn" => Angle::Turn(value),
            _ => panic!("cannot convert {} to float", value)
        }
    }
}

#[derive(Clone)]
pub enum AnglePercentage {
    Angle(Angle),
    Percentage(Percentage),
}

impl AnglePercentage {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        input
            .try_parse(|input| {
                let angle = Angle::parse(context, input)?;
                Ok(AnglePercentage::Angle(angle))
            })
            .or_else(|_err: ParseError<'i>| {
                let percentage = Percentage::parse(context, input)?;
                Ok(AnglePercentage::Percentage(percentage))
            })
    }
}
