use cssparser::{match_ignore_ascii_case, Parser, ToCss, Token, _cssparser_internal_to_lowercase};

use crate::parser::ParseError;
use crate::properties::declaration::{property_keywords_impl, PropertyDeclaration};
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;

/// https://drafts.csswg.org/css-transforms-2/#backface-visibility-property
#[derive(Clone, Debug)]
pub enum BackfaceVisibility {
	Visible,
	Hidden,
}

property_keywords_impl! { BackfaceVisibility,
	BackfaceVisibility::Visible, "visible",
	BackfaceVisibility::Hidden, "hidden",
}

pub fn parse_declared<'i, 't>(
	_context: &ParserContext,
	input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
	BackfaceVisibility::parse(input).map(PropertyDeclaration::BackfaceVisibility)
}
