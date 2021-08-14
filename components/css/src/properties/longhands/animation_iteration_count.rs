use cssparser::{Parser, Token};

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::number::Number;

#[derive(Clone)]
pub enum SingleAnimationIterationCount {
    Number(Number),
    Infinite,
}

impl SingleAnimationIterationCount {
    pub fn parse<'i, 't>(
        _context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        let token = input.next()?.clone();
        Ok(match token {
            Token::Ident(ident) if ident.eq_ignore_ascii_case("infinite") => {
                SingleAnimationIterationCount::Infinite
            },
            Token::Number { value, .. } => {
                SingleAnimationIterationCount::Number(Number::new(value))
            },
            _ => return Err(input.new_custom_error(StyleParseErrorKind::UnexpectedToken(token))),
        })
    }
}

#[derive(Clone)]
pub struct AnimationIterationCount {
    iteration_count: Vec<SingleAnimationIterationCount>,
}

impl AnimationIterationCount {
    /// https://drafts.csswg.org/css-animations/#animation-iteration-count
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
