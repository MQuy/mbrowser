use cssparser::{match_ignore_ascii_case, Parser, ToCss, Token, _cssparser_internal_to_lowercase};

use crate::parser::ParseError;
use crate::properties::declaration::{property_keywords_impl, PropertyDeclaration};
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;

#[derive(Clone)]
pub enum AlignItems {
    Stretch,
    FlexStart,
    FlexEnd,
    Center,
    Baseline,
}

property_keywords_impl! { AlignItems,
    AlignItems::Stretch, "stretch",
    AlignItems::FlexStart, "flex-start",
    AlignItems::FlexEnd, "flex-end",
    AlignItems::Center, "center",
    AlignItems::Baseline, "baseline",
}

pub fn parse_declared<'i, 't>(
    _context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
    AlignItems::parse(input).map(PropertyDeclaration::AlignItems)
}
