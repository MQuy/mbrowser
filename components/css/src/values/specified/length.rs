use std::fmt::Write;

use cssparser::{
	CowRcStr, Parser, ToCss, Token, _cssparser_internal_to_lowercase, match_ignore_ascii_case,
};
use regex::Regex;

use super::number::NonNegativeNumber;
use super::percentage::Percentage;
use crate::parser::ParseError;
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::generics::length::{
	GenericLengthOrAuto, GenericLengthOrNone, GenericLengthOrNumber,
	GenericLengthPercentageNumberOrAuto, GenericLengthPercentageNumberOrNormal,
	GenericLengthPercentageOrAuto, GenericLengthPercentageOrNormal, GenericRectOrAuto, GenericSize,
	Rect,
};
use crate::values::generics::number::NonNegative;
use crate::values::{AllowedNumericType, CSSFloat};

/// <https://drafts.csswg.org/css-values/#lengths>
#[derive(Clone, Debug, PartialEq)]
pub enum Length {
	NoCalc(NoCalcLength),
}

impl Length {
	#[inline]
	pub fn parse_non_negative<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		Length::parse_internal(context, input, AllowedNumericType::NonNegative)
	}

	pub fn parse<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		Length::parse_internal(context, input, AllowedNumericType::All)
	}

	pub fn parse_internal<'i, 't>(
		_context: &ParserContext,
		input: &mut Parser<'i, 't>,
		num_context: AllowedNumericType,
	) -> Result<Self, ParseError<'i>> {
		let location = input.current_source_location();
		let token = input.next()?.clone();
		match token {
			Token::Dimension {
				value, ref unit, ..
			} if num_context.is_ok(value) => NoCalcLength::parse(unit, value)
				.map(Length::NoCalc)
				.map_err(|()| location.new_unexpected_token_error(token.clone())),
			Token::Number { value, .. } if num_context.is_ok(value) => {
				if value != 0.0 {
					return Err(location.new_custom_error(StyleParseErrorKind::UnspecifiedError));
				}
				Ok(Length::NoCalc(NoCalcLength::Absolute(AbsoluteLength::Px(
					value,
				))))
			},
			ref t => return Err(location.new_unexpected_token_error(t.clone())),
		}
	}

	pub fn zero() -> Length {
		Length::NoCalc(NoCalcLength::Absolute(AbsoluteLength::Px(0.0)))
	}
}

impl From<&str> for Length {
	fn from(text: &str) -> Self {
		let regex = Regex::new(r"px|in|cm|mm|q|pt|pc|em|ex|ch|vw|vh|vmin|vmax").unwrap();
		let index = regex.find(text).unwrap().start();
		let (value, unit) = (&text[..index], &text[index..]);
		let value = value.parse::<f32>().unwrap();
		Length::NoCalc(match_ignore_ascii_case! { unit,
			"px" => NoCalcLength::Absolute(AbsoluteLength::Px(value)),
			"in" => NoCalcLength::Absolute(AbsoluteLength::In(value)),
			"cm" => NoCalcLength::Absolute(AbsoluteLength::Cm(value)),
			"mm" => NoCalcLength::Absolute(AbsoluteLength::Mm(value)),
			"q" => NoCalcLength::Absolute(AbsoluteLength::Q(value)),
			"pt" => NoCalcLength::Absolute(AbsoluteLength::Pt(value)),
			"pc" => NoCalcLength::Absolute(AbsoluteLength::Pc(value)),
			"em" => NoCalcLength::FontRelative(FontRelativeLength::Em(value)),
			"ex" => NoCalcLength::FontRelative(FontRelativeLength::Ex(value)),
			"ch" => NoCalcLength::FontRelative(FontRelativeLength::Ch(value)),
			"vw" => NoCalcLength::ViewportPercentage(ViewportPercentageLength::Vw(value)),
			"vh" => NoCalcLength::ViewportPercentage(ViewportPercentageLength::Vh(value)),
			"vmin" => NoCalcLength::ViewportPercentage(ViewportPercentageLength::Vmin(value)),
			"vmax" => NoCalcLength::ViewportPercentage(ViewportPercentageLength::Vmax(value)),
			_ => panic!("unit {} is not supported", unit),
		})
	}
}

impl ToCss for Length {
	fn to_css<W>(&self, dest: &mut W) -> core::fmt::Result
	where
		W: std::fmt::Write,
	{
		match self {
			Length::NoCalc(length) => length.to_css(dest),
		}
	}
}

/// <https://drafts.csswg.org/css-values/#lengths>
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NoCalcLength {
	Absolute(AbsoluteLength),
	FontRelative(FontRelativeLength),
	ViewportPercentage(ViewportPercentageLength),
}

impl NoCalcLength {
	pub fn parse(unit: &CowRcStr, value: CSSFloat) -> Result<Self, ()> {
		Ok(match_ignore_ascii_case! { &unit,
			"px" => NoCalcLength::Absolute(AbsoluteLength::Px(value)),
			"in" => NoCalcLength::Absolute(AbsoluteLength::In(value)),
			"cm" => NoCalcLength::Absolute(AbsoluteLength::Cm(value)),
			"mm" => NoCalcLength::Absolute(AbsoluteLength::Mm(value)),
			"q" => NoCalcLength::Absolute(AbsoluteLength::Q(value)),
			"pt" => NoCalcLength::Absolute(AbsoluteLength::Pt(value)),
			"pc" => NoCalcLength::Absolute(AbsoluteLength::Pc(value)),
			"em" => NoCalcLength::FontRelative(FontRelativeLength::Em(value)),
			"ex" => NoCalcLength::FontRelative(FontRelativeLength::Ex(value)),
			"ch" => NoCalcLength::FontRelative(FontRelativeLength::Ch(value)),
			"vw" => NoCalcLength::ViewportPercentage(ViewportPercentageLength::Vw(value)),
			"vh" => NoCalcLength::ViewportPercentage(ViewportPercentageLength::Vh(value)),
			"vmin" => NoCalcLength::ViewportPercentage(ViewportPercentageLength::Vmin(value)),
			"vmax" => NoCalcLength::ViewportPercentage(ViewportPercentageLength::Vmax(value)),
			_ => return Err(()),
		})
	}
}

impl ToCss for NoCalcLength {
	fn to_css<W>(&self, dest: &mut W) -> core::fmt::Result
	where
		W: std::fmt::Write,
	{
		match self {
			NoCalcLength::Absolute(absolute) => absolute.to_css(dest),
			NoCalcLength::FontRelative(font) => font.to_css(dest),
			NoCalcLength::ViewportPercentage(viewport) => viewport.to_css(dest),
		}
	}
}

/// <https://drafts.csswg.org/css-values/#absolute-length>
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AbsoluteLength {
	/// An absolute length in pixels (px)
	Px(CSSFloat),
	/// An absolute length in inches (in)
	In(CSSFloat),
	/// An absolute length in centimeters (cm)
	Cm(CSSFloat),
	/// An absolute length in millimeters (mm)
	Mm(CSSFloat),
	/// An absolute length in quarter-millimeters (q)
	Q(CSSFloat),
	/// An absolute length in points (pt)
	Pt(CSSFloat),
	/// An absolute length in pica (pc)
	Pc(CSSFloat),
}

impl ToCss for AbsoluteLength {
	fn to_css<W>(&self, dest: &mut W) -> core::fmt::Result
	where
		W: std::fmt::Write,
	{
		let (unit, value) = match self {
			AbsoluteLength::Px(value) => ("px", value),
			AbsoluteLength::In(value) => ("in", value),
			AbsoluteLength::Cm(value) => ("cm", value),
			AbsoluteLength::Mm(value) => ("mm", value),
			AbsoluteLength::Q(value) => ("q", value),
			AbsoluteLength::Pt(value) => ("pt", value),
			AbsoluteLength::Pc(value) => ("pc", value),
		};
		dest.write_fmt(format_args!("{}{}", value, unit))
	}
}

/// <https://drafts.csswg.org/css-values/#font-relative-lengths>
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FontRelativeLength {
	/// A "em" value: https://drafts.csswg.org/css-values/#em
	Em(CSSFloat),
	/// A "ex" value: https://drafts.csswg.org/css-values/#ex
	Ex(CSSFloat),
	/// A "ch" value: https://drafts.csswg.org/css-values/#ch
	Ch(CSSFloat),
	/// A "rem" value: https://drafts.csswg.org/css-values/#rem
	Rem(CSSFloat),
}

impl ToCss for FontRelativeLength {
	fn to_css<W>(&self, dest: &mut W) -> core::fmt::Result
	where
		W: Write,
	{
		let (unit, value) = match self {
			FontRelativeLength::Em(value) => ("em", value),
			FontRelativeLength::Ex(value) => ("ex", value),
			FontRelativeLength::Ch(value) => ("ch", value),
			FontRelativeLength::Rem(value) => ("rem", value),
		};
		dest.write_fmt(format_args!("{}{}", value, unit))
	}
}

/// <https://drafts.csswg.org/css-values/#viewport-relative-lengths>
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ViewportPercentageLength {
	/// A vw unit: https://drafts.csswg.org/css-values/#vw
	Vw(CSSFloat),
	/// A vh unit: https://drafts.csswg.org/css-values/#vh
	Vh(CSSFloat),
	/// <https://drafts.csswg.org/css-values/#vmin>
	Vmin(CSSFloat),
	/// <https://drafts.csswg.org/css-values/#vmax>
	Vmax(CSSFloat),
}

impl ToCss for ViewportPercentageLength {
	fn to_css<W>(&self, dest: &mut W) -> core::fmt::Result
	where
		W: Write,
	{
		let (unit, value) = match self {
			ViewportPercentageLength::Vw(value) => ("vw", value),
			ViewportPercentageLength::Vh(value) => ("vh", value),
			ViewportPercentageLength::Vmin(value) => ("vmin", value),
			ViewportPercentageLength::Vmax(value) => ("vmax", value),
		};
		dest.write_fmt(format_args!("{}{}", value, unit))
	}
}

/// https://www.w3.org/TR/css-values-4/#typedef-length-percentage
/// <length-percentage> = [ <length> | <percentage> ]
#[derive(Clone, Debug)]
pub enum LengthPercentage {
	Length(Length),
	Percentage(Percentage),
}

impl LengthPercentage {
	pub fn parse<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		Self::parse_internal(context, input, AllowedNumericType::All)
	}

	pub fn parse_non_negative<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		Self::parse_internal(context, input, AllowedNumericType::NonNegative)
	}

	pub fn parse_internal<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
		num_context: AllowedNumericType,
	) -> Result<Self, ParseError<'i>> {
		input
			.try_parse(|input| {
				let length = Length::parse_internal(context, input, num_context)?;
				Ok(LengthPercentage::Length(length))
			})
			.or_else(|_err: ParseError<'i>| {
				let percentage = Percentage::parse(context, input)?;
				Ok(LengthPercentage::Percentage(percentage))
			})
	}
}

impl From<&str> for LengthPercentage {
	fn from(text: &str) -> Self {
		match text.find(|ch| ch == '%') {
			Some(index) => {
				let value = text[..index].parse::<f32>().unwrap();
				LengthPercentage::Percentage(Percentage::new(value))
			},
			None => LengthPercentage::Length(text.into()),
		}
	}
}

impl ToCss for LengthPercentage {
	fn to_css<W>(&self, dest: &mut W) -> core::fmt::Result
	where
		W: Write,
	{
		match self {
			LengthPercentage::Length(value) => value.to_css(dest),
			LengthPercentage::Percentage(value) => value.to_css(dest),
		}
	}
}

/// value = <length [0, ∞]>
pub type NonNegativeLength = NonNegative<Length>;

impl NonNegativeLength {
	pub fn new(value: Length) -> Self {
		NonNegative::<Length>(value)
	}

	pub fn parse<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		let length = Length::parse_non_negative(context, input)?;
		Ok(Self(length))
	}
}

impl From<&str> for NonNegativeLength {
	fn from(text: &str) -> Self {
		assert!(text.chars().nth(0).unwrap() != '-');
		NonNegativeLength::new(text.into())
	}
}

/// value = <length [0, ∞]> | <percentage>
pub type NonNegativeLengthPercentage = NonNegative<LengthPercentage>;

impl NonNegativeLengthPercentage {
	pub fn parse<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		let length_percentage = LengthPercentage::parse_non_negative(context, input)?;
		Ok(Self(length_percentage))
	}
}

pub type LengthOrAuto = GenericLengthOrAuto<Length>;

impl LengthOrAuto {
	pub fn parse<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		Self::parse_with(input, |input| Length::parse(context, input))
	}
}

/// value = <length> | none
pub type NonNegativeLengthOrNone = GenericLengthOrNone<NonNegativeLength>;

impl NonNegativeLengthOrNone {
	pub fn parse<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		Self::parse_with(input, |input| NonNegativeLength::parse(context, input))
	}
}

/// value = <length> | <percentage> | auto
pub type LengthPercentageOrAuto = GenericLengthPercentageOrAuto<LengthPercentage>;

impl LengthPercentageOrAuto {
	pub fn parse<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		Self::parse_with(input, |input| LengthPercentage::parse(context, input))
	}

	pub fn zero() -> Self {
		Self::LengthPercentage(LengthPercentage::Length(Length::zero()))
	}
}

pub type NonNegativeLengthOrAuto = GenericLengthPercentageOrAuto<NonNegativeLength>;

impl NonNegativeLengthOrAuto {
	pub fn parse<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		Self::parse_with(input, |input| NonNegativeLength::parse(context, input))
	}
}

/// value = <length [0, ∞]> | <percentage> | <number [0, ∞]> | auto
pub type NonNegativeLengthPercentageNumberOrAuto =
	GenericLengthPercentageNumberOrAuto<NonNegativeLengthPercentage, NonNegativeNumber>;

impl NonNegativeLengthPercentageNumberOrAuto {
	pub fn parse<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		Self::parse_with(
			input,
			|input| NonNegativeLengthPercentage::parse(context, input),
			|input| NonNegativeNumber::parse(context, input),
		)
	}
}

pub type Size = GenericSize<NonNegativeLengthPercentage>;

impl Size {
	pub fn parse<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		Self::parse_with(input, |input| {
			NonNegativeLengthPercentage::parse(context, input)
		})
	}
}

pub type LengthPercentageOrNormal = GenericLengthPercentageOrNormal<LengthPercentage>;

impl LengthPercentageOrNormal {
	pub fn parse<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		Self::parse_with(input, |input| LengthPercentage::parse(context, input))
	}
}

pub type NonNegativeLengthPercentageOrNormal =
	GenericLengthPercentageOrNormal<NonNegativeLengthPercentage>;

impl NonNegativeLengthPercentageOrNormal {
	pub fn parse<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		Self::parse_with(input, |input| {
			NonNegativeLengthPercentage::parse(context, input)
		})
	}
}

pub type NonNegativeLengthPercentageNumberOrNormal =
	GenericLengthPercentageNumberOrNormal<NonNegativeNumber, NonNegativeLengthPercentage>;

impl NonNegativeLengthPercentageNumberOrNormal {
	pub fn parse<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		Self::parse_with(
			input,
			|input| NonNegativeNumber::parse(context, input),
			|input| NonNegativeLengthPercentage::parse(context, input),
		)
	}
}

pub type NonNegativeLengthOrNumber = GenericLengthOrNumber<NonNegativeNumber, NonNegativeLength>;

impl NonNegativeLengthOrNumber {
	pub fn parse<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		Self::parse_with(
			input,
			|input| NonNegativeNumber::parse(context, input),
			|input| NonNegativeLength::parse(context, input),
		)
	}
}

pub type NonNegativeLengthOrNumberRect = Rect<NonNegativeLengthOrNumber>;

impl NonNegativeLengthOrNumberRect {
	pub fn parse<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		Rect::parse_with(input, |input| {
			NonNegativeLengthOrNumber::parse(context, input)
		})
	}
}

pub type LengthOrAutoRectAuto = GenericRectOrAuto<GenericLengthOrAuto<Length>>;

impl LengthOrAutoRectAuto {
	pub fn parse<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		Self::parse_with(input, |input| {
			GenericLengthOrAuto::<Length>::parse_with(input, |input| Length::parse(context, input))
		})
	}
}

#[derive(Clone, Debug)]
pub struct Pair<T>(pub T, pub T);

impl<T: Clone> Pair<T> {
	pub fn new(left: T, right: T) -> Pair<T> {
		Pair::<T>(left, right)
	}

	pub fn parse_with<'i, 't, F>(
		input: &mut Parser<'i, 't>,
		item_parser: F,
	) -> Result<Self, ParseError<'i>>
	where
		F: Fn(&mut Parser<'i, 't>) -> Result<T, ParseError<'i>>,
	{
		let first = item_parser(input)?;
		let second = item_parser(input);
		if let Ok(second) = second {
			Ok(Self(first, second))
		} else {
			Ok(Self(first.clone(), first))
		}
	}
}

impl<T: ToCss> ToCss for Pair<T> {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		self.0.to_css(dest)?;
		dest.write_char(' ')?;
		self.1.to_css(dest)
	}
}
