use cssparser::Parser;

use super::percentage::Ratio;
use crate::parser::ParseError;
use crate::stylesheets::stylesheet::ParserContext;

#[derive(Clone, PartialEq)]
#[repr(C, u8)]
pub enum PreferredRatio {
    None,
    Ratio(Ratio),
}

impl PreferredRatio {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        todo!()
    }
}
