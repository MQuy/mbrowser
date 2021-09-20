use cssparser::{Parser, ToCss};

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::color::Color;

/// https://drafts.csswg.org/css-ui/#outline-color
#[derive(Clone, Debug)]
pub enum OutlineColor {
	Invert,
	Color(Color),
}

impl OutlineColor {
	pub fn parse<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		input
			.try_parse(|input| {
				input.expect_ident_matching("invert")?;
				Ok(OutlineColor::Invert)
			})
			.or_else(|_err: ParseError<'i>| {
				let color = Color::parse(context, input)?;
				Ok(OutlineColor::Color(color))
			})
	}
}

impl ToCss for OutlineColor {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		match self {
			OutlineColor::Invert => dest.write_str("invert"),
			OutlineColor::Color(value) => value.to_css(dest),
		}
	}
}

pub fn parse_declared<'i, 't>(
	context: &ParserContext,
	input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
	OutlineColor::parse(context, input).map(PropertyDeclaration::OutlineColor)
}
