use cssparser::Parser;

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::properties::declaration_block::SourcePropertyDeclaration;
use crate::properties::longhands::border_image_repeat::BorderImageRepeat;
use crate::properties::longhands::border_image_slice::BorderImageSlice;
use crate::properties::longhands::border_image_source::BorderImageSource;
use crate::properties::longhands::border_image_width::BorderImageWidth;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::specified::length::NonNegativeLengthOrNumberRect;

pub struct Longhands {
	pub border_image_outset: NonNegativeLengthOrNumberRect,
	pub border_image_repeat: BorderImageRepeat,
	pub border_image_slice: BorderImageSlice,
	pub border_image_source: BorderImageSource,
	pub border_image_width: BorderImageWidth,
}

pub fn parse_value<'i, 't>(context: &ParserContext, input: &mut Parser<'i, 't>) -> Result<Longhands, ParseError<'i>> {
	todo!()
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
			declarations.push(PropertyDeclaration::BorderImageOutset(longhands.border_image_outset));
			declarations.push(PropertyDeclaration::BorderImageRepeat(longhands.border_image_repeat));
			declarations.push(PropertyDeclaration::BorderImageSlice(longhands.border_image_slice));
			declarations.push(PropertyDeclaration::BorderImageSource(longhands.border_image_source));
			declarations.push(PropertyDeclaration::BorderImageWidth(longhands.border_image_width));
		})
}
