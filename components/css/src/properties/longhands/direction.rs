use cssparser::{match_ignore_ascii_case, Parser, ToCss, Token, _cssparser_internal_to_lowercase};

use crate::parser::ParseError;
use crate::properties::declaration::{property_keywords_impl, PropertyDeclaration};
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;

/// https://drafts.csswg.org/css-writing-modes/#direction
#[derive(Clone, Debug)]
pub enum Direction {
	Ltr,
	Rtl,
}

property_keywords_impl! { Direction,
	Direction::Ltr, "ltr",
	Direction::Rtl, "rtl",
}

pub fn parse_declared<'i, 't>(
	_context: &ParserContext,
	input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
	Direction::parse(input).map(PropertyDeclaration::Direction)
}
