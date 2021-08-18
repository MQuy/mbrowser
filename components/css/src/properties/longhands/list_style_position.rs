use cssparser::{match_ignore_ascii_case, Parser, ToCss, Token, _cssparser_internal_to_lowercase};

use crate::parser::ParseError;
use crate::properties::declaration::{property_keywords_impl, PropertyDeclaration};
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;

#[derive(Clone)]
pub enum ListStylePosition {
    Outside,
    Inside,
}

property_keywords_impl! { ListStylePosition,
    ListStylePosition::Outside, "outside",
    ListStylePosition::Inside, "inside",
}

pub fn parse_declared<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
    ListStylePosition::parse(input).map(PropertyDeclaration::ListStylePosition)
}
