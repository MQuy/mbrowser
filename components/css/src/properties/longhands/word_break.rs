use cssparser::{match_ignore_ascii_case, Parser, ToCss, Token, _cssparser_internal_to_lowercase};

use crate::parser::ParseError;
use crate::properties::declaration::{property_keywords_impl, PropertyDeclaration};
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;

/// https://drafts.csswg.org/css-text/#word-break-property
#[derive(Clone, Debug)]
#[repr(u8)]
pub enum WordBreak {
	Normal,
	BreakAll,
	KeepAll,
	BreakWord,
}

property_keywords_impl! { WordBreak,
	WordBreak::Normal, "normal",
	WordBreak::BreakAll, "break-all",
	WordBreak::KeepAll, "keep-all",
	WordBreak::BreakWord, "break-word",
}

pub fn parse_declared<'i, 't>(
	context: &ParserContext,
	input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
	WordBreak::parse(input).map(PropertyDeclaration::WordBreak)
}
