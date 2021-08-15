use cssparser::{Delimiter, Parser};

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::number::Integer;
use crate::values::CustomIdent;

#[derive(Clone)]
pub struct CounterIncrementValue {
    name: CustomIdent,
    value: Option<Integer>,
}

impl CounterIncrementValue {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        let name = input.try_parse(|input| {
            let location = input.current_source_location();
            let ident = input.expect_ident()?;
            CustomIdent::from_ident(location, ident, &["none"])
        })?;
        let value = input.try_parse(|input| Integer::parse(context, input)).ok();
        Ok(CounterIncrementValue { name, value })
    }
}

#[derive(Clone)]
pub enum CounterIncrement {
    None,
    Counter(Vec<CounterIncrementValue>),
}

impl CounterIncrement {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        input
            .try_parse(|input| {
                input.expect_ident_matching("none")?;
                Ok(CounterIncrement::None)
            })
            .or_else(|_err: ParseError<'i>| {
                let mut counters = vec![CounterIncrementValue::parse(context, input)?];
                input.try_parse(|input| {
                    input.parse_until_before(Delimiter::Semicolon, |input| {
                        let value = CounterIncrementValue::parse(context, input)?;
                        counters.push(value);
                        Ok(())
                    })
                })?;
                Ok(CounterIncrement::Counter(counters))
            })
    }
}

pub fn parse_declared<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
    CounterIncrement::parse(context, input).map(PropertyDeclaration::CounterIncrement)
}
