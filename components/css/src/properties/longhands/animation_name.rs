use cssparser::{Parser, ToCss};

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::animation::KeyframesName;

#[derive(Clone)]
pub struct AnimationName {
    names: Vec<Option<KeyframesName>>,
}

impl AnimationName {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        let names = input.parse_comma_separated(|input| {
            input
                .try_parse(|input| {
                    input.expect_ident_matching("none")?;
                    Ok(None)
                })
                .or_else(|_err: ParseError<'i>| {
                    let name = KeyframesName::parse(context, input)?;
                    Ok(Some(name))
                })
        })?;
        Ok(AnimationName { names })
    }
}

impl ToCss for AnimationName {
    fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
    where
        W: std::fmt::Write,
    {
        let names: Vec<String> = self
            .names
            .iter()
            .map(|name| {
                name.as_ref()
                    .map_or("none".to_string(), |v| v.to_css_string())
            })
            .collect();
        dest.write_str(&names.join(", "))
    }
}

pub fn parse_declared<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
    AnimationName::parse(context, input).map(PropertyDeclaration::AnimationName)
}
