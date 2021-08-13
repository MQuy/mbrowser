use cssparser::Parser;

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::position::PreferredRatio;

#[derive(Clone)]
pub struct AspectRatio {
    pub auto: bool,
    pub ratio: PreferredRatio,
}

impl AspectRatio {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<AspectRatio, ParseError<'i>> {
        todo!()
    }
}

pub fn parse_declared<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
    AspectRatio::parse(context, input).map(PropertyDeclaration::AspectRatio)
}
