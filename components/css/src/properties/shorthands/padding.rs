use cssparser::Parser;

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::properties::declaration_block::SourcePropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::shortcut_for_four_values;
use crate::values::specified::length::NonNegativeLengthPercentage;

pub struct Longhands {
	pub padding_top: NonNegativeLengthPercentage,
	pub padding_right: NonNegativeLengthPercentage,
	pub padding_bottom: NonNegativeLengthPercentage,
	pub padding_left: NonNegativeLengthPercentage,
}

shortcut_for_four_values!(
	Longhands,
	padding_top,
	padding_right,
	padding_bottom,
	padding_left,
	NonNegativeLengthPercentage
);

/// Parse the given shorthand and fill the result into the
/// `declarations` vector.
pub fn parse_into<'i, 't>(
	declarations: &mut SourcePropertyDeclaration,
	_context: &ParserContext,
	input: &mut Parser<'i, 't>,
) -> Result<(), ParseError<'i>> {
	input
		.parse_entirely(|input| Longhands::parse_values(input))
		.map(|longhands| {
			declarations.push(PropertyDeclaration::PaddingTop(longhands.padding_top));
			declarations.push(PropertyDeclaration::PaddingRight(longhands.padding_right));
			declarations.push(PropertyDeclaration::PaddingBottom(longhands.padding_bottom));
			declarations.push(PropertyDeclaration::PaddingLeft(longhands.padding_left));
		})
}
