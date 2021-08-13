use cssparser::{match_ignore_ascii_case, Parser, Token, _cssparser_internal_to_lowercase};

use crate::parser::ParseError;
use crate::properties::declaration::{property_keywords_impl, PropertyDeclaration};
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;

#[derive(Clone)]
pub enum PointerEvents {
    Auto,
    None,
}

property_keywords_impl! { PointerEvents,
    PointerEvents::Auto, "auto",
    PointerEvents::None, "none",
}

pub fn parse_declared<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
    PointerEvents::parse(input).map(PropertyDeclaration::PointerEvents)
}
