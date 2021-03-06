use cssparser::{Parser, ToCss};

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::specified::image::Image;

/// https://drafts.csswg.org/css-lists/#image-markers
#[derive(Clone, Debug)]
pub enum ListStyleImage {
	None,
	Image(Image),
}

impl ListStyleImage {
	pub fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i>> {
		input
			.try_parse(|input| {
				input.expect_ident_matching("none")?;
				Ok(ListStyleImage::None)
			})
			.or_else(|_err: ParseError<'i>| {
				let image = input.try_parse(|input| Image::parse(input))?;
				Ok(ListStyleImage::Image(image))
			})
	}
}

impl ToCss for ListStyleImage {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		match self {
			ListStyleImage::None => dest.write_str("none"),
			ListStyleImage::Image(image) => image.to_css(dest),
		}
	}
}

pub fn parse_declared<'i, 't>(
	_context: &ParserContext,
	input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
	ListStyleImage::parse(input).map(PropertyDeclaration::ListStyleImage)
}
