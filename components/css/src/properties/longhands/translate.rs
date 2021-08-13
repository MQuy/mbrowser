use cssparser::Parser;

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::length::{Length, LengthPercentage};

#[derive(Clone)]
pub enum Translate {
    None,
    LengthPercentage(LengthPercentage, LengthPercentageWithLength),
}

impl Translate {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        todo!()
    }
}

#[derive(Clone)]
pub struct LengthPercentageWithLength {
    length_percentage: LengthPercentage,
    length: Option<Length>,
}

impl LengthPercentageWithLength {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        todo!()
    }
}

#[derive(Clone)]
pub struct LengthPercentageComponent {
    length: Vec<LengthPercentageWithLength>,
}

impl LengthPercentageComponent {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        let length = input
            .parse_comma_separated(|input| LengthPercentageWithLength::parse(context, input))?;
        Ok(LengthPercentageComponent { length })
    }
}

pub fn parse_declared<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
    Translate::parse(context, input).map(PropertyDeclaration::Translate)
}
