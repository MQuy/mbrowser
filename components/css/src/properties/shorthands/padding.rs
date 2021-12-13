use cssparser::Parser;

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::properties::declaration_block::SourcePropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::specified::length::NonNegativeLengthPercentage;

pub struct Longhands {
	pub padding_top: NonNegativeLengthPercentage,
	pub padding_right: NonNegativeLengthPercentage,
	pub padding_bottom: NonNegativeLengthPercentage,
	pub padding_left: NonNegativeLengthPercentage,
}

impl Longhands {
	pub fn from_single_value(value: NonNegativeLengthPercentage) -> Self {
		Longhands {
			padding_top: value.clone(),
			padding_right: value.clone(),
			padding_bottom: value.clone(),
			padding_left: value,
		}
	}

	pub fn from_two_values(first: NonNegativeLengthPercentage, second: NonNegativeLengthPercentage) -> Self {
		Longhands {
			padding_top: first.clone(),
			padding_right: second.clone(),
			padding_bottom: first,
			padding_left: second,
		}
	}

	pub fn from_three_values(
		first: NonNegativeLengthPercentage,
		second: NonNegativeLengthPercentage,
		third: NonNegativeLengthPercentage,
	) -> Self {
		Longhands {
			padding_top: first,
			padding_right: second.clone(),
			padding_bottom: third,
			padding_left: second,
		}
	}
}

pub fn parse_value<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Longhands, ParseError<'i>> {
	let first = NonNegativeLengthPercentage::parse(input)?;
	let second = if let Ok(second) = NonNegativeLengthPercentage::parse(input) {
		second
	} else {
		return Ok(Longhands::from_single_value(first));
	};
	let third = if let Ok(third) = NonNegativeLengthPercentage::parse(input) {
		third
	} else {
		return Ok(Longhands::from_two_values(first, second));
	};
	let forth = if let Ok(forth) = NonNegativeLengthPercentage::parse(input) {
		forth
	} else {
		return Ok(Longhands::from_three_values(first, second, third));
	};
	Ok(Longhands {
		padding_top: first,
		padding_right: second,
		padding_bottom: third,
		padding_left: forth,
	})
}

/// Parse the given shorthand and fill the result into the
/// `declarations` vector.
pub fn parse_into<'i, 't>(
	declarations: &mut SourcePropertyDeclaration,
	_context: &ParserContext,
	input: &mut Parser<'i, 't>,
) -> Result<(), ParseError<'i>> {
	input.parse_entirely(|input| parse_value(input)).map(|longhands| {
		declarations.push(PropertyDeclaration::PaddingTop(longhands.padding_top));
		declarations.push(PropertyDeclaration::PaddingRight(longhands.padding_right));
		declarations.push(PropertyDeclaration::PaddingBottom(longhands.padding_bottom));
		declarations.push(PropertyDeclaration::PaddingLeft(longhands.padding_left));
	})
}
