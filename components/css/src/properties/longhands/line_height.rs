use cssparser::Parser;

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::length::NonNegativeLengthPercentage;
use crate::values::number::NonNegativeNumber;

#[derive(Clone)]
#[repr(C, u8)]
pub enum LineHeight {
    Normal,
    Number(NonNegativeNumber),
    Length(NonNegativeLengthPercentage),
}

impl LineHeight {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<LineHeight, ParseError<'i>> {
        todo!()
    }
}

pub fn parse_declared<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
    LineHeight::parse(context, input).map(PropertyDeclaration::LineHeight)
}
