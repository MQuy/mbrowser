use cssparser::Parser;

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::number::Number;
use crate::values::specified::angle::Angle;

#[derive(Clone)]
#[repr(C, u8)]
pub enum Rotate {
    None,
    Rotate(Angle),
    Rotate3D(Number, Number, Number, Angle),
}

impl Rotate {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Rotate, ParseError<'i>> {
        todo!()
    }
}

pub fn parse_declared<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
    Rotate::parse(context, input).map(PropertyDeclaration::Rotate)
}
