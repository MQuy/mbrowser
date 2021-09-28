use cssparser::{Parser, ToCss};

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::specified::image::Image;

#[derive(Clone, Debug)]
pub enum BgImage {
	None,
	Image(Image),
}

impl BgImage {
	pub fn parse<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		input
			.try_parse(|input| {
				input.expect_ident_matching("none")?;
				Ok(BgImage::None)
			})
			.or_else(|_err: ParseError<'i>| {
				let image = Image::parse(context, input)?;
				Ok(BgImage::Image(image))
			})
	}
}

impl ToCss for BgImage {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		match self {
			BgImage::None => dest.write_str("none"),
			BgImage::Image(image) => image.to_css(dest),
		}
	}
}

/// https://drafts.csswg.org/css-backgrounds/#background-image
#[derive(Clone, Debug)]
pub struct BackgroundImage {
	images: Vec<BgImage>,
}

impl BackgroundImage {
	pub fn parse<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		let images = input.parse_comma_separated(|input| BgImage::parse(context, input))?;
		Ok(BackgroundImage { images })
	}
}

impl ToCss for BackgroundImage {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		let images: Vec<String> = self.images.iter().map(|v| v.to_css_string()).collect();
		dest.write_str(&images.join(", "))
	}
}

pub fn parse_declared<'i, 't>(
	context: &ParserContext,
	input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
	BackgroundImage::parse(context, input).map(PropertyDeclaration::BackgroundImage)
}
