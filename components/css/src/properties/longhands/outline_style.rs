use cssparser::Parser;

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::layout::BorderStyle;

#[derive(Clone)]
#[repr(C, u8)]
pub enum OutlineStyle {
    Auto,
    BorderStyle(BorderStyle),
}

impl OutlineStyle {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<OutlineStyle, ParseError<'i>> {
        todo!()
    }
}

pub fn parse_declared<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
    OutlineStyle::parse(context, input).map(PropertyDeclaration::OutlineStyle)
}
