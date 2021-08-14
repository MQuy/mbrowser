use cssparser::Parser;

use super::length::{NonNegativeLength, NonNegativeLengthPercentage, Pair};
use crate::parser::ParseError;
use crate::stylesheets::stylesheet::ParserContext;

#[derive(Clone)]
pub enum BorderSideWidth {
    Thin,
    Medium,
    Thick,
    Length(NonNegativeLength),
}

impl BorderSideWidth {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        todo!()
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct GenericBorderCornerRadius<L>(pub Pair<L>);

pub type BorderCornerRadius = GenericBorderCornerRadius<NonNegativeLengthPercentage>;

impl BorderCornerRadius {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        todo!()
    }
}
