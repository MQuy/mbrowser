use cssparser::Parser;

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::generics::length::Rect;
use crate::values::specified::length::NonNegativeLengthPercentageNumberOrAuto;

/// https://drafts.csswg.org/css-backgrounds/#border-image-width
pub type BorderImageWidth = Rect<NonNegativeLengthPercentageNumberOrAuto>;

impl BorderImageWidth {
	pub fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i>> {
		Rect::parse_with(input, |input| NonNegativeLengthPercentageNumberOrAuto::parse(input))
	}
}

pub fn parse_declared<'i, 't>(
	_context: &ParserContext,
	input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
	BorderImageWidth::parse(input).map(PropertyDeclaration::BorderImageWidth)
}
