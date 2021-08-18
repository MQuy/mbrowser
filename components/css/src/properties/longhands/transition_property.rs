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

impl SingleTransitionProperty {
    pub fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i>> {
        input
            .try_parse(|input| {
                input.expect_ident_matching("all")?;
                Ok(SingleTransitionProperty::All)
            })
            .or_else(|_err: ParseError<'i>| {
                let ident = CustomIdent::parse(input)?;
                Ok(SingleTransitionProperty::Ident(ident))
            })
    }
}

#[derive(Clone)]
pub struct TransitionProperty {
    properties: Vec<SingleTransitionProperty>,
}

impl TransitionProperty {
    pub fn parse<'i, 't>(
        _context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<TransitionProperty, ParseError<'i>> {
        input
            .try_parse(|input| {
                input.expect_ident_matching("none")?;
                Ok(TransitionProperty { properties: vec![] })
            })
            .or_else(|_err: ParseError<'i>| {
                let properties =
                    input.parse_comma_separated(|input| SingleTransitionProperty::parse(input))?;
                Ok(TransitionProperty { properties })
            })
    }
}

pub fn parse_declared<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
    TransitionProperty::parse(context, input).map(PropertyDeclaration::TransitionProperty)
}
