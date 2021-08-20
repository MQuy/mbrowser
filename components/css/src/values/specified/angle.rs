use cssparser::Parser;

use crate::parser::ParseError;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::percentage::Percentage;
use crate::values::CSSFloat;

#[derive(Clone, PartialEq)]
pub enum Angle {
    Deg(CSSFloat),
    Grad(CSSFloat),
    Rad(CSSFloat),
    Turn(CSSFloat),
}

impl Angle {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        todo!()
    }
}

impl From<&str> for Angle {
    fn from(_: &str) -> Self {
        todo!()
    }
}

#[derive(Clone)]
pub enum AnglePercentage {
    Angle(Angle),
    Percentage(Percentage),
}

impl AnglePercentage {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        todo!()
    }
}
