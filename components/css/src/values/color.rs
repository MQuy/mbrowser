use cssparser::Parser;

use crate::parser::ParseError;
use crate::stylesheets::stylesheet::ParserContext;

#[derive(Clone, PartialEq)]
#[repr(C)]
pub struct RGBA {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

#[derive(Clone, PartialEq)]
#[repr(C)]
pub struct ComplexColorRatios {
    pub bg: f32,
    pub fg: f32,
}

#[derive(Clone, PartialEq)]
#[repr(C, u8)]
pub enum GenericColor<RGBA> {
    Numeric(RGBA),
    CurrentColor,
    Complex {
        color: RGBA,
        ratios: ComplexColorRatios,
    },
}

#[derive(Clone, PartialEq)]
pub enum Color {
    CurrentColor,
    Transparent,
    Numeric {
        parsed: RGBA,
        authored: Option<Box<str>>,
    },
    Complex(GenericColor<RGBA>),
}

impl Color {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Color, ParseError<'i>> {
        todo!()
    }
}
