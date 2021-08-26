use cssparser::{Parser, ToCss};

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::image::Image;

/// https://drafts.csswg.org/css-backgrounds/#the-border-image-source
#[derive(Clone)]
pub enum BorderImageSource {
	None,
	Image(Image),
}

impl BorderImageSource {
	pub fn parse<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		input
			.try_parse(|input| {
				input.expect_ident_matching("none")?;
				Ok(BorderImageSource::None)
			})
			.or_else(|_err: ParseError<'i>| {
				let image = Image::parse(context, input)?;
				Ok(BorderImageSource::Image(image))
			})
	}
}

impl ToCss for BorderImageSource {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		match self {
			BorderImageSource::None => dest.write_str("none"),
			BorderImageSource::Image(value) => value.to_css(dest),
		}
	}
}

pub fn parse_declared<'i, 't>(
	context: &ParserContext,
	input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
	BorderImageSource::parse(context, input).map(PropertyDeclaration::BorderImageSource)
}
