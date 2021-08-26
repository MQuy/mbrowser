use cssparser::Parser;

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::properties::declaration_block::SourcePropertyDeclaration;
use crate::properties::longhands;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::color::Color;

pub struct Longhands {
	pub background_color: Color,
	pub background_position_x: longhands::background_position_x::BackgroundPositionX,
	pub background_position_y: longhands::background_position_y::BackgroundPositionY,
	pub background_repeat: longhands::background_repeat::BackgroundRepeat,
	pub background_attachment: longhands::background_attachment::BackgroundAttachment,
	pub background_image: longhands::background_image::BackgroundImage,
	pub background_size: longhands::background_size::BackgroundSize,
	pub background_origin: longhands::background_origin::BackgroundOrigin,
	pub background_clip: longhands::background_clip::BackgroundClip,
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
			declarations.push(PropertyDeclaration::BackgroundColor(
				longhands.background_color,
			));
			declarations.push(PropertyDeclaration::BackgroundPositionX(
				longhands.background_position_x,
			));
			declarations.push(PropertyDeclaration::BackgroundPositionY(
				longhands.background_position_y,
			));
			declarations.push(PropertyDeclaration::BackgroundRepeat(
				longhands.background_repeat,
			));
			declarations.push(PropertyDeclaration::BackgroundAttachment(
				longhands.background_attachment,
			));
			declarations.push(PropertyDeclaration::BackgroundImage(
				longhands.background_image,
			));
			declarations.push(PropertyDeclaration::BackgroundSize(
				longhands.background_size,
			));
			declarations.push(PropertyDeclaration::BackgroundOrigin(
				longhands.background_origin,
			));
			declarations.push(PropertyDeclaration::BackgroundClip(
				longhands.background_clip,
			));
		})
}
