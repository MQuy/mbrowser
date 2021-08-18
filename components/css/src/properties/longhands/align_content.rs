use cssparser::{match_ignore_ascii_case, Parser, ToCss, Token, _cssparser_internal_to_lowercase};

use crate::parser::ParseError;
use crate::properties::declaration::{property_keywords_impl, PropertyDeclaration};
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;

#[derive(Clone)]
pub enum AlignContent {
    Stretch,
    FlexStart,
    FlexEnd,
    Center,
    SpaceBetween,
    SpaceAround,
}

property_keywords_impl! { AlignContent,
    AlignContent::Stretch, "stretch",
    AlignContent::FlexStart, "flex-start",
    AlignContent::FlexEnd, "flex-end",
    AlignContent::Center, "center",
    AlignContent::SpaceBetween, "space-between",
    AlignContent::SpaceAround, "space-around",
}

pub fn parse_declared<'i, 't>(
    _context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
    AlignContent::parse(input).map(PropertyDeclaration::AlignContent)
}
