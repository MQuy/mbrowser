use cssparser::{Parser, Token};

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::CustomIdent;

#[derive(Clone)]
pub enum KeyframesName {
    None,
    Ident(CustomIdent),
    QuotedString(String),
}

#[derive(Clone)]
pub struct AnimationName {
    names: Vec<KeyframesName>,
}

impl KeyframesName {
    pub fn parse<'i, 't>(
        _context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        todo!()
    }
}

pub fn parse<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<AnimationName, ParseError<'i>> {
    todo!()
}

pub fn parse_declared<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
    parse(context, input).map(PropertyDeclaration::AnimationName)
}
