use cssparser::{match_ignore_ascii_case, Parser, ToCss, Token, _cssparser_internal_to_lowercase};

use crate::parser::ParseError;
use crate::properties::declaration::{property_keywords_impl, PropertyDeclaration};
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;

/// https://drafts.csswg.org/css-text/#text-align-property
#[derive(Clone, Debug)]
pub enum TextAlign {
	Start,
	End,
	Left,
	Right,
	Center,
	Justify,
	MatchParent,
	JustifyAll,
}

property_keywords_impl! { TextAlign,
	TextAlign::Start, "start",
	TextAlign::Left, "left",
	TextAlign::Right, "right",
	TextAlign::Center, "center",
	TextAlign::Justify, "justify",
	TextAlign::End, "end",
	TextAlign::MatchParent, "match-parent",
	TextAlign::JustifyAll, "justify-all",
}

pub fn parse_declared<'i, 't>(
	_context: &ParserContext,
	input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
	TextAlign::parse(input).map(PropertyDeclaration::TextAlign)
}
