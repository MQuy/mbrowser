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

impl SingleAnimationIterationCount {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        todo!()
    }
}

#[derive(Clone)]
pub struct AnimationIterationCount {
    iteration_count: Vec<SingleAnimationIterationCount>,
}

impl AnimationIterationCount {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        let iteration_count = input
            .parse_comma_separated(|input| SingleAnimationIterationCount::parse(context, input))?;
        Ok(AnimationIterationCount { iteration_count })
    }
}

pub fn parse_declared<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
    AnimationIterationCount::parse(context, input).map(PropertyDeclaration::AnimationIterationCount)
}
