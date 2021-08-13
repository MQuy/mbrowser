use cssparser::Parser;

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::time::Time;

#[derive(Clone)]
pub struct AnimationDuration {
    durations: Vec<Time>,
}

impl AnimationDuration {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        let times = input.parse_comma_separated(|input| Time::parse(context, input))?;
        Ok(AnimationDuration { durations: times })
    }
}

pub fn parse_declared<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
    AnimationDuration::parse(context, input).map(PropertyDeclaration::AnimationDuration)
}
