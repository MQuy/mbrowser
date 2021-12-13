use cssparser::Parser;

use super::length::{NonNegativeLengthPercentage, Pair};
use crate::parser::ParseError;
use crate::values::generics::border::GenericBorderCornerRadius;

pub type BorderCornerRadius = GenericBorderCornerRadius<NonNegativeLengthPercentage>;

impl BorderCornerRadius {
	pub fn new(horizontal: NonNegativeLengthPercentage, vertical: NonNegativeLengthPercentage) -> Self {
		GenericBorderCornerRadius(Pair::new(horizontal, vertical))
	}

	pub fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i>> {
		let pair = Pair::parse_with(input, |input| NonNegativeLengthPercentage::parse(input))?;
		Ok(BorderCornerRadius::new(pair.0, pair.1))
	}
}
