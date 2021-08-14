use cssparser::{Parser, ToCss, Token};

use super::number::NonNegativeNumber;
use super::CSSFloat;
use crate::parser::ParseError;
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;

#[derive(Clone, Debug)]
pub struct Percentage {
    value: CSSFloat,
}

impl Percentage {
    /// https://drafts.csswg.org/css-values/#percentages
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        let token = input.next()?.clone();
        match token {
            Token::Dimension {
                value, ref unit, ..
            } if unit == &"%" => Ok(Percentage { value }),
            _ => Err(input.new_custom_error(StyleParseErrorKind::UnexpectedToken(token))),
        }
    }
}

/// A computed <ratio> value.
#[derive(Clone, PartialEq, Debug)]
pub struct Ratio(pub NonNegativeNumber, pub NonNegativeNumber);

impl Ratio {
    /// Parse a ratio.
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        let first_value = NonNegativeNumber::parse(context, input)?;
        input
            .try_parse(|input| {
                input.expect_delim('/')?;
                let second_value = NonNegativeNumber::parse(context, input)?;
                Ok(Ratio(first_value.clone(), second_value))
            })
            .or_else(|_err: ParseError<'i>| Ok(Ratio(first_value, NonNegativeNumber::new(1.0))))
    }
}

impl ToCss for Ratio {
    fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
    where
        W: std::fmt::Write,
    {
        self.0.to_css(dest)?;
        dest.write_str(" / ")?;
        self.1.to_css(dest)
    }
}
