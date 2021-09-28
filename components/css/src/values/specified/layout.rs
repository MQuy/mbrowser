use cssparser::{match_ignore_ascii_case, Parser, ToCss, Token, _cssparser_internal_to_lowercase};

use crate::parser::ParseError;
use crate::properties::declaration::property_keywords_impl;
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::CSSFloat;

#[derive(Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum LineStyle {
	Hidden,
	None,
	Inset,
	Groove,
	Outset,
	Ridge,
	Dotted,
	Dashed,
	Solid,
	Double,
}

property_keywords_impl! { LineStyle,
	LineStyle::Hidden, "hidden",
	LineStyle::None, "none",
	LineStyle::Inset, "inset",
	LineStyle::Groove, "groove",
	LineStyle::Outset, "outset",
	LineStyle::Ridge, "ridge",
	LineStyle::Dotted, "dotted",
	LineStyle::Dashed, "dashed",
	LineStyle::Solid, "solid",
	LineStyle::Double, "double",
}

#[derive(Clone, Debug)]
#[repr(u8)]
pub enum Overflow {
	Visible,
	Hidden,
	Scroll,
	Auto,
}

property_keywords_impl! { Overflow,
	Overflow::Visible, "visible",
	Overflow::Hidden, "hidden",
	Overflow::Scroll, "scroll",
	Overflow::Auto, "auto",
}

/// A specified resolution.
#[derive(Clone, Debug, PartialEq)]
pub enum Resolution {
	/// Dots per inch.
	Dpi(CSSFloat),
	/// Dots per pixel.
	Dppx(CSSFloat),
	/// Dots per centimeter.
	Dpcm(CSSFloat),
}

impl Resolution {
	/// Parse a resolution.
	pub fn parse<'i, 't>(
		_context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		let token = input.next()?.clone();
		match token {
			cssparser::Token::Dimension { value, unit, .. } if value >= 0.0 => {
				match_ignore_ascii_case! { &unit,
					"dpi" => Ok(Resolution::Dpi(value)),
					"x" | "dppx"=> Ok(Resolution::Dppx(value)),
					"dpcm" => Ok(Resolution::Dpcm(value)),
					_ => return Err(input.new_custom_error(StyleParseErrorKind::UnexpectedDimension(unit))),
				}
			},
			ref t => return Err(input.new_unexpected_token_error(t.clone())),
		}
	}
}

impl ToCss for Resolution {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		let (unit, value) = match self {
			Resolution::Dpi(value) => ("dpi", value),
			Resolution::Dppx(value) => ("dppx", value),
			Resolution::Dpcm(value) => ("dpcm", value),
		};
		dest.write_fmt(format_args!("{}{}", value, unit))
	}
}

impl From<&str> for Resolution {
	fn from(text: &str) -> Self {
		let index = text.find(|ch: char| ch == 'x' || ch == 'd').unwrap();
		let (value, unit) = (&text[..index], &text[index..]);
		let value = value.parse::<f32>().unwrap();
		match_ignore_ascii_case! { unit,
			"dpi" => Resolution::Dpi(value),
			"x" | "dppx"=> Resolution::Dppx(value),
			"dpcm" => Resolution::Dpcm(value),
			_ => Resolution::Dppx(1.0),
		}
	}
}

#[derive(Clone, Debug)]
pub enum Box {
	BorderBox,
	PaddingBox,
	ContentBox,
}

property_keywords_impl! { Box,
	Box::BorderBox, "border-box",
	Box::PaddingBox, "padding-box",
	Box::ContentBox, "content-box",
}
