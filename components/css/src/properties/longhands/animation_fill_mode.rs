use cssparser::Parser;

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;

#[derive(Clone)]
pub enum SingleAnimationFillMode {
    None,
    Forwards,
    Backwards,
    Both,
}

#[derive(Clone)]
pub struct AnimationFillMode {
    fill_modes: Vec<SingleAnimationFillMode>,
}

pub fn parse<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<AnimationFillMode, ParseError<'i>> {
    todo!()
}

pub fn parse_declared<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
    parse(context, input).map(PropertyDeclaration::AnimationFillMode)
}
