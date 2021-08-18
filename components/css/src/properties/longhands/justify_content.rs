use cssparser::{match_ignore_ascii_case, Parser, ToCss, Token, _cssparser_internal_to_lowercase};

use crate::parser::ParseError;
use crate::properties::declaration::{property_keywords_impl, PropertyDeclaration};
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;

#[derive(Clone)]
pub enum JustifyContent {
    FlexStart,
    Stretch,
    FlexEnd,
    Center,
    SpaceBetween,
    SpaceAround,
}

property_keywords_impl! { JustifyContent,
    JustifyContent::FlexStart, "flex-start",
    JustifyContent::Stretch, "stretch",
    JustifyContent::FlexEnd, "flex-end",
    JustifyContent::Center, "center",
    JustifyContent::SpaceBetween, "space-between",
    JustifyContent::SpaceAround, "space-around",
}

pub fn parse_declared<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
    JustifyContent::parse(input).map(PropertyDeclaration::JustifyContent)
}
