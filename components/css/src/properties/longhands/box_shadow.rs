use cssparser::Parser;

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;

#[derive(Clone)]
pub enum BoxShadow {
    None,
}

impl BoxShadow {
    pub fn parse<'i, 't>(
        _context: &ParserContext,
        _input: &mut Parser<'i, 't>,
    ) -> Result<BoxShadow, ParseError<'i>> {
        todo!()
    }
}

pub fn parse_declared<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
    BoxShadow::parse(context, input).map(PropertyDeclaration::BoxShadow)
}
