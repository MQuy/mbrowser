use cssparser::{match_ignore_ascii_case, Parser, ToCss, Token, _cssparser_internal_to_lowercase};

use crate::parser::ParseError;
use crate::properties::declaration::{property_keywords_impl, PropertyDeclaration};
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;

/// https://drafts.csswg.org/css-sizing/#box-sizing
#[derive(Clone, Debug)]
pub enum BoxSizing {
	ContentBox,
	BorderBox,
}

property_keywords_impl! { BoxSizing,
	BoxSizing::ContentBox, "content-box",
	BoxSizing::BorderBox, "border-box",
}

pub fn parse_declared<'i, 't>(
	_context: &ParserContext,
	input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
	BoxSizing::parse(input).map(PropertyDeclaration::BoxSizing)
}
