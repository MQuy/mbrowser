use cssparser::Parser;

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::CustomIdent;

#[derive(Clone)]
pub enum SingleTransitionProperty {
    All,
    Ident(CustomIdent),
}

#[derive(Clone)]
pub struct TransitionProperty {
    properties: Vec<SingleTransitionProperty>,
}

pub fn parse<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<TransitionProperty, ParseError<'i>> {
    todo!()
}

pub fn parse_declared<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
    parse(context, input).map(PropertyDeclaration::TransitionProperty)
}
