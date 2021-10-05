use cssparser::Parser;

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::properties::declaration_block::SourcePropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::specified::length::LengthPercentageOrAuto;

pub struct Longhands {
	pub margin_top: LengthPercentageOrAuto,
	pub margin_right: LengthPercentageOrAuto,
	pub margin_bottom: LengthPercentageOrAuto,
	pub margin_left: LengthPercentageOrAuto,
}

impl Longhands {
	pub fn from_single_value(value: LengthPercentageOrAuto) -> Self {
		Longhands {
			margin_top: value.clone(),
			margin_right: value.clone(),
			margin_bottom: value.clone(),
			margin_left: value,
		}
	}

	pub fn from_two_values(first: LengthPercentageOrAuto, second: LengthPercentageOrAuto) -> Self {
		Longhands {
			margin_top: first.clone(),
			margin_right: second.clone(),
			margin_bottom: first,
			margin_left: second,
		}
	}

	pub fn from_three_values(
		first: LengthPercentageOrAuto,
		second: LengthPercentageOrAuto,
		third: LengthPercentageOrAuto,
	) -> Self {
		Longhands {
			margin_top: first,
			margin_right: second.clone(),
			margin_bottom: third,
			margin_left: second,
		}
	}
}

pub fn parse_value<'i, 't>(
	context: &ParserContext,
	input: &mut Parser<'i, 't>,
) -> Result<Longhands, ParseError<'i>> {
	let first = LengthPercentageOrAuto::parse(context, input)?;
	let second = if let Ok(second) = LengthPercentageOrAuto::parse(context, input) {
		second
	} else {
		return Ok(Longhands::from_single_value(first));
	};
	let third = if let Ok(third) = LengthPercentageOrAuto::parse(context, input) {
		third
	} else {
		return Ok(Longhands::from_two_values(first, second));
	};
	let forth = if let Ok(forth) = LengthPercentageOrAuto::parse(context, input) {
		forth
	} else {
		return Ok(Longhands::from_three_values(first, second, third));
	};
	Ok(Longhands {
		margin_top: first,
		margin_right: second,
		margin_bottom: third,
		margin_left: forth,
	})
}

/// Parse the given shorthand and fill the result into the
/// `declarations` vector.
pub fn parse_into<'i, 't>(
	declarations: &mut SourcePropertyDeclaration,
	context: &ParserContext,
	input: &mut Parser<'i, 't>,
) -> Result<(), ParseError<'i>> {
	input
		.parse_entirely(|input| parse_value(context, input))
		.map(|longhands| {
			declarations.push(PropertyDeclaration::MarginTop(longhands.margin_top));
			declarations.push(PropertyDeclaration::MarginRight(longhands.margin_right));
			declarations.push(PropertyDeclaration::MarginBottom(longhands.margin_bottom));
			declarations.push(PropertyDeclaration::MarginLeft(longhands.margin_left));
		})
}
