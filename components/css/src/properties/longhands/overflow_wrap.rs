use cssparser::{match_ignore_ascii_case, Parser, ToCss, Token, _cssparser_internal_to_lowercase};

use crate::parser::ParseError;
use crate::properties::declaration::{property_keywords_impl, PropertyDeclaration};
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;

/// https://drafts.csswg.org/css-text-3/#propdef-overflow-wrap
#[derive(Clone)]
pub enum OverflowWrap {
	Normal,
	BreakWord,
	Anywhere,
}

property_keywords_impl! { OverflowWrap,
	OverflowWrap::Normal, "normal",
	OverflowWrap::BreakWord, "break-word",
	OverflowWrap::Anywhere, "anywhere",
}

pub fn parse_declared<'i, 't>(
	context: &ParserContext,
	input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
	OverflowWrap::parse(input).map(PropertyDeclaration::OverflowWrap)
}
