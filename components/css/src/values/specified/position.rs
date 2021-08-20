use cssparser::Parser;

use crate::parser::ParseError;
use crate::stylesheets::stylesheet::ParserContext;

#[derive(Clone)]
pub enum Position {}

impl Position {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        todo!()
    }
}

impl From<&str> for Position {
    fn from(_: &str) -> Self {
        todo!()
    }
}
