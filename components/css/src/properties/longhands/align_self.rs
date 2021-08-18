use cssparser::{match_ignore_ascii_case, Parser, ToCss, Token, _cssparser_internal_to_lowercase};

use crate::parser::ParseError;
use crate::properties::declaration::{property_keywords_impl, PropertyDeclaration};
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;

#[derive(Clone)]
pub enum AlignSelf {
    Auto,
    Stretch,
    FlexStart,
    FlexEnd,
    Center,
    Baseline,
}

property_keywords_impl! { AlignSelf,
    AlignSelf::Auto, "auto",
    AlignSelf::Stretch, "stretch",
    AlignSelf::FlexStart, "flex-start",
    AlignSelf::FlexEnd, "flex-end",
    AlignSelf::Center, "center",
    AlignSelf::Baseline, "baseline",
}

pub fn parse_declared<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
    AlignSelf::parse(input).map(PropertyDeclaration::AlignSelf)
}
