use cssparser::{match_ignore_ascii_case, Parser, Token, _cssparser_internal_to_lowercase};

use crate::parser::ParseError;
use crate::properties::declaration::{property_keywords_impl, PropertyDeclaration};
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;

#[derive(Clone)]
pub enum Float {
    Left,
    Right,
    None,
    InlineStart,
    InlineEnd,
}

property_keywords_impl! { Float,
    Float::Left, "left",
    Float::Right, "right",
    Float::None, "none",
    Float::InlineStart, "inline-start",
    Float::InlineEnd, "inline-end",
}

pub fn parse_declared<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
    Float::parse(input).map(PropertyDeclaration::Float)
}
