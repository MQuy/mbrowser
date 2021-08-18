use cssparser::{match_ignore_ascii_case, Parser, ToCss, Token, _cssparser_internal_to_lowercase};

use crate::parser::ParseError;
use crate::properties::declaration::{property_keywords_impl, PropertyDeclaration};
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;

#[derive(Clone)]
pub enum SingleAnimationDirection {
    Normal,
    Reverse,
    Alternate,
    AlternateReverse,
}

property_keywords_impl! { SingleAnimationDirection,
    SingleAnimationDirection::Normal, "normal",
    SingleAnimationDirection::Reverse, "reverse",
    SingleAnimationDirection::Alternate, "alternate",
    SingleAnimationDirection::AlternateReverse, "laternate-reverse",
}

#[derive(Clone)]
pub struct AnimationDirection {
    directions: Vec<SingleAnimationDirection>,
}

impl AnimationDirection {
    pub fn parse<'i, 't>(
        _context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        let directions = input.parse_comma_separated(SingleAnimationDirection::parse)?;
        Ok(AnimationDirection { directions })
    }
}

pub fn parse_declared<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
    AnimationDirection::parse(context, input).map(PropertyDeclaration::AnimationDirection)
}
