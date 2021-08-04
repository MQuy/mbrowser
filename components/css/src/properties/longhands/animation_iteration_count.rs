use cssparser::Parser;

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::number::Number;

#[derive(Clone)]
pub enum SingleAnimationIterationCount {
    Number(Number),
    Infinite,
}

#[derive(Clone)]
pub struct AnimationIterationCount {
    iteration_count: Vec<SingleAnimationIterationCount>,
}

pub fn parse<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<AnimationIterationCount, ParseError<'i>> {
    todo!()
}

pub fn parse_declared<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
    parse(context, input).map(PropertyDeclaration::AnimationIterationCount)
}
