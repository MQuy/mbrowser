use cssparser::{Parser, ToCss};

use crate::parser::ParseError;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::percentage::Percentage;

#[derive(Clone, PartialEq)]
pub enum GenericNumberOrPercentage<Number> {
    Number(Number),
    Percentage(Percentage),
}

impl<N> GenericNumberOrPercentage<N> {
    pub fn parse_with<'i, 't, NP>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
        number_parser: NP,
    ) -> Result<Self, ParseError<'i>>
    where
        NP: FnOnce(&mut Parser<'i, 't>) -> Result<N, ParseError<'i>>,
    {
        input
            .try_parse(|input| {
                let value = Percentage::parse(context, input)?;
                Ok(Self::Percentage(value))
            })
            .or_else(|_err: ParseError<'i>| {
                let value = number_parser(input)?;
                Ok(Self::Number(value))
            })
    }
}

impl<N: ToCss> ToCss for GenericNumberOrPercentage<N> {
    fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
    where
        W: std::fmt::Write,
    {
        todo!()
    }
}

#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub struct NonNegative<T>(pub T);

impl<T: ToCss + Clone> ToCss for NonNegative<T> {
    fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
    where
        W: std::fmt::Write,
    {
        self.0.to_css(dest)
    }
}

#[derive(Clone, PartialEq, PartialOrd)]
pub struct GreaterThanOrEqualToOne<T>(pub T);
