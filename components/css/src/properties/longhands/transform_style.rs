use cssparser::{match_ignore_ascii_case, Parser, ToCss, Token, _cssparser_internal_to_lowercase};

use crate::parser::ParseError;
use crate::properties::declaration::{property_keywords_impl, PropertyDeclaration};
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;

/// https://drafts.csswg.org/css-transforms-2/#transform-style-property
#[derive(Clone, Debug)]
pub enum TransformStyle {
	Flat,
	Preserve3d,
}

property_keywords_impl! { TransformStyle,
	TransformStyle::Flat, "flat",
	TransformStyle::Preserve3d, "preserve-3d",
}

pub fn parse_declared<'i, 't>(
	_context: &ParserContext,
	input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
	TransformStyle::parse(input).map(PropertyDeclaration::TransformStyle)
}
