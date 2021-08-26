use cssparser::Parser;

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::properties::declaration_block::SourcePropertyDeclaration;
use crate::properties::longhands::list_style_position::ListStylePosition;
use crate::properties::longhands::list_style_type::ListStyleType;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::image::Image;

pub struct Longhands {
	pub list_style_position: ListStylePosition,
	pub list_style_image: Image,
	pub list_style_type: ListStyleType,
}

pub fn parse_value<'i, 't>(
	context: &ParserContext,
	input: &mut Parser<'i, 't>,
) -> Result<Longhands, ParseError<'i>> {
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
			declarations.push(PropertyDeclaration::ListStylePosition(
				longhands.list_style_position,
			));
			declarations.push(PropertyDeclaration::ListStyleImage(
				longhands.list_style_image,
			));
			declarations.push(PropertyDeclaration::ListStyleType(
				longhands.list_style_type,
			));
		})
}
