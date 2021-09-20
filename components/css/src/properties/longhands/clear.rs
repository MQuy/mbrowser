use cssparser::{match_ignore_ascii_case, Parser, ToCss, Token, _cssparser_internal_to_lowercase};

use crate::parser::ParseError;
use crate::properties::declaration::{property_keywords_impl, PropertyDeclaration};
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;

/// https://drafts.csswg.org/css-logical/#float-clear
#[derive(Clone, Debug)]
pub enum Clear {
	None,
	Left,
	Right,
	Both,
	InlineStart,
	InlineEnd,
}

property_keywords_impl! { Clear,
	Clear::None, "none",
	Clear::Left, "left",
	Clear::Right, "right",
	Clear::Both, "both",
	Clear::InlineStart, "inline-start",
	Clear::InlineEnd, "inline-end",
}

pub fn parse_declared<'i, 't>(
	_context: &ParserContext,
	input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
	Clear::parse(input).map(PropertyDeclaration::Clear)
}
