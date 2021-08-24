use cssparser::{match_ignore_ascii_case, Parser, ToCss, Token, _cssparser_internal_to_lowercase};

use crate::parser::ParseError;
use crate::properties::declaration::{property_keywords_impl, PropertyDeclaration};
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;

/// https://drafts.csswg.org/css-flexbox/#flex-wrap-property
#[derive(Clone)]
pub enum FlexWrap {
    Nowrap,
    Wrap,
    WrapReverse,
}

property_keywords_impl! { FlexWrap,
    FlexWrap::Nowrap, "no-wrap",
    FlexWrap::Wrap, "wrap",
    FlexWrap::WrapReverse, "wrap-reverse",
}

pub fn parse_declared<'i, 't>(
    _context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
    FlexWrap::parse(input).map(PropertyDeclaration::FlexWrap)
}
