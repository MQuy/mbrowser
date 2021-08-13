use std::iter::Zip;

use cssparser::Parser;

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::number::Integer;

#[derive(Clone)]
#[repr(C, u8)]
pub enum ZIndex {
    Integer(Integer),
    Auto,
}

impl ZIndex {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<ZIndex, ParseError<'i>> {
        todo!()
    }
}

pub fn parse_declared<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
    ZIndex::parse(context, input).map(PropertyDeclaration::ZIndex)
}
