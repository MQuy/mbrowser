use cssparser::{match_ignore_ascii_case, Parser, ToCss, Token, _cssparser_internal_to_lowercase};

use crate::parser::ParseError;
use crate::properties::declaration::{property_keywords_impl, PropertyDeclaration};
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;

/// https://drafts.csswg.org/css-flexbox/#flex-direction-property
#[derive(Clone)]
pub enum FlexDirection {
	Row,
	RowReverse,
	Column,
	ColumnReverse,
}

property_keywords_impl! { FlexDirection,
	FlexDirection::Row, "row",
	FlexDirection::RowReverse, "row-reverse",
	FlexDirection::Column, "column",
	FlexDirection::ColumnReverse, "column-reverse",
}

pub fn parse_declared<'i, 't>(
	_context: &ParserContext,
	input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
	FlexDirection::parse(input).map(PropertyDeclaration::FlexDirection)
}
