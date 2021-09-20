use cssparser::{match_ignore_ascii_case, Parser, ToCss, Token, _cssparser_internal_to_lowercase};

use crate::parser::ParseError;
use crate::properties::declaration::{property_keywords_impl, PropertyDeclaration};
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;

/// https://drafts.csswg.org/css2/#empty-cells
#[derive(Clone, Debug)]
pub enum EmptyCells {
	Show,
	Hide,
}

property_keywords_impl! { EmptyCells,
	EmptyCells::Show, "show",
	EmptyCells::Hide, "hide",
}

pub fn parse_declared<'i, 't>(
	_context: &ParserContext,
	input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
	EmptyCells::parse(input).map(PropertyDeclaration::EmptyCells)
}
