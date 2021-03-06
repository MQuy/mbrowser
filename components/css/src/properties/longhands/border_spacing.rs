use cssparser::Parser;

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::specified::length::{NonNegativeLength, Pair};

/// https://drafts.csswg.org/css2/#separated-borders
pub type BorderSpacing = Pair<NonNegativeLength>;

impl BorderSpacing {
	pub fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<BorderSpacing, ParseError<'i>> {
		Pair::parse_with(input, |input| NonNegativeLength::parse(input))
	}
}

pub fn parse_declared<'i, 't>(
	_context: &ParserContext,
	input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
	BorderSpacing::parse(input).map(PropertyDeclaration::BorderSpacing)
}
