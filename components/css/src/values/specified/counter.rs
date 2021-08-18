use cssparser::Parser;

use crate::parser::ParseError;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::generics::counter::{GenericCounter, GenericCounterOrNone};
use crate::values::number::Integer;

pub type CounterWithInteger = GenericCounterOrNone<GenericCounter<Integer>>;

impl CounterWithInteger {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        GenericCounterOrNone::parse_with(input, |input| {
            GenericCounter::parse_with(input, |input| Integer::parse(context, input))
        })
    }
}

#[derive(Clone)]
pub enum Counter {}

impl Counter {
    /// https://drafts.csswg.org/css-lists-3/#typedef-counter
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        todo!()
    }
}
