use cssparser::{match_ignore_ascii_case, Parser, Token, _cssparser_internal_to_lowercase};

use crate::parser::ParseError;
use crate::properties::declaration::{property_keywords_impl, PropertyDeclaration};
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;

#[derive(Clone)]
pub enum UnicodeBidi {
    Normal,
    Embed,
    Isolate,
    BidiOverride,
    IsolateOverride,
    Plaintext,
}

property_keywords_impl! { UnicodeBidi,
    UnicodeBidi::Normal, "normal",
    UnicodeBidi::Embed, "embed",
    UnicodeBidi::Isolate, "isolate",
    UnicodeBidi::BidiOverride, "bidi-override",
    UnicodeBidi::IsolateOverride, "isolate-override",
    UnicodeBidi::Plaintext, "plantext",
}

pub fn parse_declared<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
    UnicodeBidi::parse(input).map(PropertyDeclaration::UnicodeBidi)
}
