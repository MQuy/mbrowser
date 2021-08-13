use cssparser::Parser;

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::time::Time;

#[derive(Clone)]
pub struct TransitionDelay {
    delays: Vec<Time>,
}

impl TransitionDelay {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        let delays = input.parse_comma_separated(|input| Time::parse(context, input))?;
        Ok(TransitionDelay { delays })
    }
}

pub fn parse_declared<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
    TransitionDelay::parse(context, input).map(PropertyDeclaration::TransitionDelay)
}
