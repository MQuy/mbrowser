use std::ops::Range;

use cssparser::Parser;

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

#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub struct NonNegative<T>(pub T);

#[derive(Clone, PartialEq, PartialOrd)]
pub struct GreaterThanOrEqualToOne<T>(pub T);
