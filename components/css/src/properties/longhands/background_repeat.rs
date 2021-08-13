use cssparser::{match_ignore_ascii_case, Parser, Token, _cssparser_internal_to_lowercase};

use crate::parser::ParseError;
use crate::properties::declaration::{property_keywords_impl, PropertyDeclaration};
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;

#[derive(Clone)]
pub enum BackgroundRepeatKeyword {
    Repeat,
    Space,
    Round,
    NoRepeat,
}

property_keywords_impl! { BackgroundRepeatKeyword,
    BackgroundRepeatKeyword::Repeat, "repeat",
    BackgroundRepeatKeyword::Space, "space",
    BackgroundRepeatKeyword::Round, "round",
    BackgroundRepeatKeyword::NoRepeat, "no-repeat",
}

#[derive(Clone)]
pub struct BackgroundRepeat(pub BackgroundRepeatKeyword, pub BackgroundRepeatKeyword);

impl BackgroundRepeat {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        todo!()
    }
}

pub fn parse_declared<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
    BackgroundRepeat::parse(context, input).map(PropertyDeclaration::BackgroundRepeat)
}
