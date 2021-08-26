use cssparser::Parser;

use super::generics::border::GenericBorderCornerRadius;
use super::length::{NonNegativeLengthPercentage, Pair};
use crate::parser::ParseError;
use crate::stylesheets::stylesheet::ParserContext;

pub type BorderCornerRadius = GenericBorderCornerRadius<NonNegativeLengthPercentage>;

impl BorderCornerRadius {
	pub fn new(
		horizontal: NonNegativeLengthPercentage,
		vertical: NonNegativeLengthPercentage,
	) -> Self {
		GenericBorderCornerRadius(Pair::new(horizontal, vertical))
	}

	pub fn parse<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		let pair = Pair::parse_with(input, |input| {
			NonNegativeLengthPercentage::parse(context, input)
		})?;
		Ok(BorderCornerRadius::new(pair.0, pair.1))
	}
}
