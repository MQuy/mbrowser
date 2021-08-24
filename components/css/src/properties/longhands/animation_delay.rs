use cssparser::{Parser, ToCss};

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::time::Time;

/// https://drafts.csswg.org/css-animations-1/#animation-delay
#[derive(Clone)]
pub struct AnimationDelay {
    times: Vec<Time>,
}

impl AnimationDelay {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        let times = input.parse_comma_separated(|input| Time::parse(context, input))?;
        Ok(AnimationDelay { times })
    }
}

impl ToCss for AnimationDelay {
    fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
    where
        W: std::fmt::Write,
    {
        let times: Vec<String> = self.times.iter().map(|v| v.to_css_string()).collect();
        dest.write_str(&times.join(", "))
    }
}

pub fn parse_declared<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
    AnimationDelay::parse(context, input).map(PropertyDeclaration::AnimationDelay)
}
