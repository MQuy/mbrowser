use cssparser::Parser;

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::animation::KeyframesName;

#[derive(Clone)]
pub struct AnimationName {
    names: Vec<KeyframesName>,
}

impl AnimationName {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        let names = input.parse_comma_separated(|input| KeyframesName::parse(context, input))?;
        Ok(AnimationName { names })
    }
}

pub fn parse_declared<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
    AnimationName::parse(context, input).map(PropertyDeclaration::AnimationName)
}
