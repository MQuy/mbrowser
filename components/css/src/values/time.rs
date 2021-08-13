use cssparser::Parser;

use super::CSSFloat;
use crate::parser::ParseError;
use crate::stylesheets::stylesheet::ParserContext;

#[derive(Clone)]
pub enum TimeUnit {
    Second,
    Millisecond,
}

#[derive(Clone)]
pub struct Time {
    amount: CSSFloat,
    unit: TimeUnit,
}

impl Time {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        todo!()
    }
}
