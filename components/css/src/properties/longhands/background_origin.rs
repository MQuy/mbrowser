use cssparser::Parser;

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::layout::Box;

#[derive(Clone)]
pub struct BackgroundOrigin {
    boxes: Vec<Box>,
}

impl BackgroundOrigin {
    pub fn parse<'i, 't>(
        _context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        let boxes = input.parse_comma_separated(Box::parse)?;
        Ok(BackgroundOrigin { boxes })
    }
}

pub fn parse_declared<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
    BackgroundOrigin::parse(context, input).map(PropertyDeclaration::BackgroundOrigin)
}
