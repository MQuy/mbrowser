use cssparser::{match_ignore_ascii_case, Parser, ToCss, Token, _cssparser_internal_to_lowercase};

use crate::parser::ParseError;
use crate::properties::declaration::{property_keywords_impl, PropertyDeclaration};
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;

/// https://drafts.csswg.org/css2/#visibility
#[derive(Clone)]
pub enum Visibility {
    Visible,
    Hidden,
    Collapse,
}

property_keywords_impl! { Visibility,
    Visibility::Visible, "visible",
    Visibility::Hidden, "hidden",
    Visibility::Collapse, "collapse",
}

pub fn parse_declared<'i, 't>(
    _context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
    Visibility::parse(input).map(PropertyDeclaration::Visibility)
}
