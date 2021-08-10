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
        todo!()
    }
}

impl ToCss for Ratio {
    fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
    where
        W: std::fmt::Write,
    {
        todo!()
    }
}
