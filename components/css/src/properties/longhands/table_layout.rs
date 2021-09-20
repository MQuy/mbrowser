use cssparser::{match_ignore_ascii_case, Parser, ToCss, Token, _cssparser_internal_to_lowercase};

use crate::parser::ParseError;
use crate::properties::declaration::{property_keywords_impl, PropertyDeclaration};
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;

/// https://drafts.csswg.org/css2/#width-layout
#[derive(Clone, Debug)]
pub enum TableLayout {
	Auto,
	Fixed,
}

property_keywords_impl! { TableLayout,
	TableLayout::Auto, "auto",
	TableLayout::Fixed, "fixed",
}

pub fn parse_declared<'i, 't>(
	_context: &ParserContext,
	input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
	TableLayout::parse(input).map(PropertyDeclaration::TableLayout)
}
