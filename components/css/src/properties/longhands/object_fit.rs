use cssparser::{match_ignore_ascii_case, Parser, ToCss, Token, _cssparser_internal_to_lowercase};

use crate::parser::ParseError;
use crate::properties::declaration::{property_keywords_impl, PropertyDeclaration};
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;

/// https://drafts.csswg.org/css-images/#the-object-fit
#[derive(Clone, Debug)]
pub enum ObjectFit {
	Fill,
	Contain,
	Cover,
	None,
	ScaleDown,
}

property_keywords_impl! { ObjectFit,
	ObjectFit::Fill, "fill",
	ObjectFit::Contain, "contain",
	ObjectFit::Cover, "cover",
	ObjectFit::None, "none",
	ObjectFit::ScaleDown, "scale-down",
}

pub fn parse_declared<'i, 't>(
	_context: &ParserContext,
	input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
	ObjectFit::parse(input).map(PropertyDeclaration::ObjectFit)
}
