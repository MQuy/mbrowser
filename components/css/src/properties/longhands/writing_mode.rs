use cssparser::{match_ignore_ascii_case, Parser, ToCss, Token, _cssparser_internal_to_lowercase};

use crate::parser::ParseError;
use crate::properties::declaration::{property_keywords_impl, PropertyDeclaration};
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;

/// https://drafts.csswg.org/css-writing-modes/#block-flow
#[derive(Clone)]
pub enum WritingMode {
    HorizontalTb,
    VerticalRl,
    VerticalLr,
    SidewaysRl,
    SidewaysLr,
}

property_keywords_impl! { WritingMode,
    WritingMode::HorizontalTb, "horizontal-tb",
    WritingMode::VerticalRl, "vertical-rl",
    WritingMode::VerticalLr, "vertical-lr",
    WritingMode::SidewaysRl, "sideways-rl",
    WritingMode::SidewaysLr, "sideways-lr",
}

pub fn parse_declared<'i, 't>(
    _context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
    WritingMode::parse(input).map(PropertyDeclaration::WritingMode)
}
