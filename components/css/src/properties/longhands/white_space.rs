use cssparser::{match_ignore_ascii_case, Parser, ToCss, Token, _cssparser_internal_to_lowercase};

use crate::parser::ParseError;
use crate::properties::declaration::{property_keywords_impl, PropertyDeclaration};
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;

#[derive(Clone)]
pub enum WhiteSpace {
    Normal,
    Pre,
    Nowrap,
    PreWrap,
    PreLine,
}

property_keywords_impl! { WhiteSpace,
    WhiteSpace::Normal, "normal",
    WhiteSpace::Pre, "pre",
    WhiteSpace::Nowrap, "nowrap",
    WhiteSpace::PreWrap, "pre-wrap",
    WhiteSpace::PreLine, "pre-line",
}

pub fn parse_declared<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
    WhiteSpace::parse(input).map(PropertyDeclaration::WhiteSpace)
}
