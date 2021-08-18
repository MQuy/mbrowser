use cssparser::{Delimiter, Parser};

use crate::parser::ParseError;
use crate::values::CustomIdent;

#[derive(Clone)]
pub enum GenericCounterOrNone<Counter> {
    None,
    Counter(Vec<Counter>),
}

impl<C> GenericCounterOrNone<C> {
    pub fn parse_with<'i, 't, F>(
        input: &mut Parser<'i, 't>,
        item_parser: F,
    ) -> Result<Self, ParseError<'i>>
    where
        F: for<'a, 'b> Fn(&mut Parser<'a, 'b>) -> Result<C, ParseError<'a>>,
    {
        input
            .try_parse(|input| {
                input.expect_ident_matching("none")?;
                Ok(GenericCounterOrNone::None)
            })
            .or_else(|_err: ParseError<'i>| {
                let mut counters = vec![item_parser(input)?];
                input.try_parse(|input| {
                    input.parse_until_before(Delimiter::Semicolon, |input| {
                        let value = item_parser(input)?;
                        counters.push(value);
                        Ok(())
                    })
                })?;
                Ok(GenericCounterOrNone::Counter(counters))
            })
    }
}

#[derive(Clone)]
pub struct GenericCounter<I> {
    name: CustomIdent,
    value: Option<I>,
}

impl<I> GenericCounter<I> {
    pub fn parse_with<'i, 't, F>(
        input: &mut Parser<'i, 't>,
        item_parser: F,
    ) -> Result<Self, ParseError<'i>>
    where
        F: Fn(&mut Parser<'i, 't>) -> Result<I, ParseError<'i>>,
    {
        let name = CustomIdent::parse(input)?;
        let value = input.try_parse(|input| item_parser(input)).ok();
        Ok(GenericCounter { name, value })
    }
}

#[derive(Clone)]
pub struct GenericReversedCounter<I> {
    name: CustomIdent,
    value: Option<I>,
}

impl<I> GenericReversedCounter<I> {
    pub fn parse_with<'i, 't, F>(
        input: &mut Parser<'i, 't>,
        item_parser: F,
    ) -> Result<Self, ParseError<'i>>
    where
        F: Fn(&mut Parser<'i, 't>) -> Result<I, ParseError<'i>>,
    {
        input.expect_function_matching("reversed")?;
        let name = input.parse_nested_block(|input| CustomIdent::parse(input))?;
        let value = input.try_parse(|input| item_parser(input)).ok();
        Ok(GenericReversedCounter { name, value })
    }
}
