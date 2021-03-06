use std::ops::Range;

use cssparser::{Parser, ToCss, Token};

use super::number::NonNegativeNumber;
use crate::parser::ParseError;
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::values::CSSFloat;

/// https://drafts.csswg.org/css-values/#percentages
#[derive(Clone, Debug, PartialEq)]
pub struct Percentage {
	value: CSSFloat,
}

impl Percentage {
	pub fn new(value: CSSFloat) -> Self {
		Percentage { value }
	}

	pub fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i>> {
		let token = input.next()?.clone();
		match token {
			Token::Percentage { unit_value, .. } => Ok(Percentage { value: unit_value }),
			_ => Err(input.new_custom_error(StyleParseErrorKind::UnexpectedToken(token))),
		}
	}

	pub fn to_value(&self, range: &Range<f32>) -> CSSFloat {
		(range.end - range.start) * self.value + range.start
	}
}

impl ToCss for Percentage {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		dest.write_fmt(format_args!("{}%", self.value * 100.0))
	}
}

/// A computed <ratio> value.
#[derive(Clone, PartialEq, Debug)]
pub struct Ratio(pub NonNegativeNumber, pub NonNegativeNumber);

impl Ratio {
	/// Parse a ratio.
	pub fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i>> {
		let first_value = NonNegativeNumber::parse(input)?;
		input
			.try_parse(|input| {
				input.expect_delim('/')?;
				let second_value = NonNegativeNumber::parse(input)?;
				Ok(Ratio(first_value.clone(), second_value))
			})
			.or_else(|_err: ParseError<'i>| Ok(Ratio(first_value, NonNegativeNumber::new(1.0))))
	}
}

impl ToCss for Ratio {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		self.0.to_css(dest)?;
		dest.write_str(" / ")?;
		self.1.to_css(dest)
	}
}
