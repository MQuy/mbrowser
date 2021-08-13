use cssparser::{match_ignore_ascii_case, Parser, Token, _cssparser_internal_to_lowercase};

use crate::parser::ParseError;
use crate::properties::declaration::{property_keywords_impl, PropertyDeclaration};
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;

#[derive(Clone)]
pub enum Position {
    Static,
    Absolute,
    Relative,
    Fixed,
    Sticky,
}

property_keywords_impl! { Position,
    Position::Static, "static",
    Position::Absolute, "absolute",
    Position::Relative, "relative",
    Position::Fixed, "fixed",
    Position::Sticky, "sticky",
}

pub fn parse_declared<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
    Position::parse(input).map(PropertyDeclaration::Position)
}
