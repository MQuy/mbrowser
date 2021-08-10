use cssparser::{Parser, ToCss};

use super::number::NonNegativeNumber;
use super::CSSFloat;
use crate::parser::ParseError;
use crate::stylesheets::stylesheet::ParserContext;

#[derive(Clone, Debug)]
pub struct Percentage {
    value: CSSFloat,
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
