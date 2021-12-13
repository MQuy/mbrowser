use std::f32::consts::PI;

use cssparser::{Parser, ToCss, Token, _cssparser_internal_to_lowercase, match_ignore_ascii_case};
use regex::Regex;

use super::percentage::Percentage;
use crate::parser::ParseError;
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::values::CSSFloat;

/// https://drafts.csswg.org/css-values-4/#angle-value
#[derive(Clone, Debug, PartialEq)]
pub enum Angle {
	Deg(CSSFloat),
	Grad(CSSFloat),
	Rad(CSSFloat),
	Turn(CSSFloat),
}

impl Angle {
	pub fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i>> {
		macro_rules! return_unexpected_token {
			($location:tt, $token:tt) => {
				return Err($location.new_custom_error(StyleParseErrorKind::UnexpectedToken($token.clone())))
			};
		}

		let location = input.current_source_location();
		let token = input.next()?;
		Ok(match token {
			Token::Dimension { value, unit, .. } => match_ignore_ascii_case! { unit,
				"deg" => Angle::Deg(*value),
				"grad" => Angle::Grad(*value),
				"rad" => Angle::Rad(*value),
				"turn" => Angle::Turn(*value),
				_ => return_unexpected_token!(location, token)
			},
			_ => return_unexpected_token!(location, token),
		})
	}

	pub fn to_deg(&self) -> CSSFloat {
		match *self {
			Angle::Deg(value) => value,
			Angle::Grad(value) => value * 360.0 / 400.0,
			Angle::Rad(value) => value * 180.0 / PI,
			Angle::Turn(value) => value * 360.0,
		}
	}
}

impl From<&str> for Angle {
	fn from(text: &str) -> Self {
		let regex = Regex::new(r"deg|grad|rad|turn").unwrap();
		let index = regex.find(text).unwrap().start();
		let (value, unit) = (&text[..index], &text[index..]);
		let value = value.parse::<f32>().unwrap();
		match_ignore_ascii_case! { unit,
			"deg" => Angle::Deg(value),
			"grad" => Angle::Grad(value),
			"rad" => Angle::Rad(value),
			"turn" => Angle::Turn(value),
			_ => panic!("cannot convert {} to float", value)
		}
	}
}

impl ToCss for Angle {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		let (value, unit) = match self {
			Angle::Deg(value) => (value, "deg"),
			Angle::Grad(value) => (value, "grad"),
			Angle::Rad(value) => (value, "rad"),
			Angle::Turn(value) => (value, "turn"),
		};
		dest.write_fmt(format_args!("{}{}", value, unit))
	}
}

/// https://drafts.csswg.org/css-values-4/#typedef-angle-percentage
#[derive(Clone, Debug)]
pub enum AnglePercentage {
	Angle(Angle),
	Percentage(Percentage),
}

impl AnglePercentage {
	pub fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i>> {
		input
			.try_parse(|input| {
				let angle = Angle::parse(input)?;
				Ok(AnglePercentage::Angle(angle))
			})
			.or_else(|_err: ParseError<'i>| {
				let percentage = Percentage::parse(input)?;
				Ok(AnglePercentage::Percentage(percentage))
			})
	}
}

impl ToCss for AnglePercentage {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		match self {
			AnglePercentage::Angle(value) => value.to_css(dest),
			AnglePercentage::Percentage(value) => value.to_css(dest),
		}
	}
}
