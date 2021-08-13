use cssparser::Parser;

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::number::PositiveInteger;

#[derive(Clone)]
pub enum ColumnCount {
    Integer(PositiveInteger),
    Auto,
}

impl ColumnCount {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<ColumnCount, ParseError<'i>> {
        todo!()
    }
}

pub fn parse_declared<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
    ColumnCount::parse(context, input).map(PropertyDeclaration::ColumnCount)
}
