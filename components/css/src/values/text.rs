use cssparser::Parser;

use crate::parser::ParseError;
use crate::stylesheets::stylesheet::ParserContext;

#[derive(Clone)]
pub enum Spacing<Value> {
    Normal,
    Value(Value),
}

impl<T> Spacing<T> {
    pub fn parse_with<'i, 't, P>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
        item_parser: P,
    ) -> Result<Self, ParseError<'i>>
    where
        P: Fn(&mut Parser<'i, 't>) -> Result<T, ParseError<'i>>,
    {
        input
            .try_parse(|input| {
                input.expect_ident_matching("normal")?;
                Ok(Self::Normal)
            })
            .or_else(|_err: ParseError<'i>| {
                let value = item_parser(input)?;
                Ok(Self::Value(value))
            })
    }
}
