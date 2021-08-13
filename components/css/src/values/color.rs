use cssparser::Parser;

use crate::parser::ParseError;
use crate::stylesheets::stylesheet::ParserContext;

#[derive(Clone)]
#[repr(C)]
pub struct RGBA {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

#[derive(Clone)]
#[repr(C)]
pub struct ComplexColorRatios {
    pub bg: f32,
    pub fg: f32,
}

#[derive(Clone)]
#[repr(C, u8)]
pub enum GenericColor<RGBA> {
    Numeric(RGBA),
    CurrentColor,
    Complex {
        color: RGBA,
        ratios: ComplexColorRatios,
    },
}

#[derive(Clone)]
pub enum Color {
    CurrentColor,
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
