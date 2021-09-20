use cssparser::{match_ignore_ascii_case, Parser, ToCss, Token, _cssparser_internal_to_lowercase};

use crate::parser::ParseError;
use crate::properties::declaration::{property_keywords_impl, PropertyDeclaration};
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;

/// https://drafts.csswg.org/css-text/#text-justify-property
#[derive(Clone, Debug)]
pub enum TextJustify {
	Auto,
	None,
	InterWord,
	InterCharacter,
}

property_keywords_impl! { TextJustify,
	TextJustify::Auto, "auto",
	TextJustify::None, "none",
	TextJustify::InterWord, "inter-word",
	TextJustify::InterCharacter, "inter-character",
}

pub fn parse_declared<'i, 't>(
	_context: &ParserContext,
	input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
	TextJustify::parse(input).map(PropertyDeclaration::TextJustify)
}
