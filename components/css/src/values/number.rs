use std::cmp::Ordering;
use std::fmt::Display;
use std::ops::Range;

use cssparser::{Parser, ToCss};

use super::generics::number::{GenericNumberOrPercentage, GreaterThanOrEqualToOne, NonNegative};
use super::percentage::Percentage;
use super::CSSFloat;
use crate::parser::ParseError;
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;

/// https://www.w3.org/TR/css-values-4/#numbers
#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub struct Number {
	value: CSSFloat,
}

impl Number {
	pub fn new(value: CSSFloat) -> Self {
		Number { value }
	}

	/// Parse a float.
	pub fn parse<'i, 't>(
		_context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		let value = input.expect_number()?;
		Ok(Number { value })
	}

	pub fn parse_non_negative<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		let value = Number::parse(context, input)?;
		if value < 0 {
			Err(input.new_custom_error(StyleParseErrorKind::UnspecifiedError))
		} else {
			Ok(value)
		}
	}

	pub fn parse_in_range<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
		from: f32,
		to: f32,
	) -> Result<Self, ParseError<'i>> {
		let value = Number::parse(context, input)?;
		if from <= value.get() && value.get() <= to {
			Ok(value)
		} else {
			Err(input.new_custom_error(StyleParseErrorKind::UnspecifiedError))
		}
	}

	pub fn get(&self) -> f32 {
		self.value
	}
}

impl From<i32> for Number {
	fn from(value: i32) -> Self {
		Number::new(value as f32)
	}
}

impl From<f32> for Number {
	fn from(value: f32) -> Self {
		Number::new(value)
	}
}

impl PartialEq<i32> for Number {
	fn eq(&self, other: &i32) -> bool {
		self.get() as i32 == *other
	}
}

impl PartialOrd<i32> for Number {
	fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
		let value = self.get() as i32;
		if value > *other {
			Some(Ordering::Greater)
		} else if value < *other {
			Some(Ordering::Less)
		} else {
			Some(Ordering::Equal)
		}
	}
}

impl Display for Number {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_fmt(format_args!("{}", self.value))
	}
}

impl ToCss for Number {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		dest.write_str(&self.to_string())
	}
}

pub type PositiveInteger = GreaterThanOrEqualToOne<Integer>;

#[derive(Clone)]
pub struct Integer(i32);

impl Integer {
	/// Parse a non-negative integer.
	pub fn parse_non_negative<'i, 't>(
		_context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		let value = input.expect_integer()?;
		if value < 0 {
			return Err(input.new_custom_error(StyleParseErrorKind::UnspecifiedError));
		}
		Ok(Integer(value))
	}

	pub fn parse<'i, 't>(
		_context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		let value = input.expect_integer()?;
		Ok(Integer(value))
	}

	pub fn parse_in_range<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
		range: Range<i32>,
	) -> Result<Self, ParseError<'i>> {
		let value = Integer::parse(context, input)?;
		if range.start <= value.get() && value.get() <= range.end {
			Ok(value)
		} else {
			Err(input.new_custom_error(StyleParseErrorKind::UnspecifiedError))
		}
	}

	pub fn parse_from<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
		from: i32,
	) -> Result<Self, ParseError<'i>> {
		let value = Integer::parse(context, input)?;
		if value.get() >= from {
			Ok(value)
		} else {
			Err(input.new_custom_error(StyleParseErrorKind::UnspecifiedError))
		}
	}

	pub fn get(&self) -> i32 {
		self.0
	}
}

impl ToCss for Integer {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		dest.write_fmt(format_args!("{}", self.0))
	}
}

pub type NonNegativeNumber = NonNegative<Number>;

impl NonNegativeNumber {
	pub fn new(val: CSSFloat) -> Self {
		NonNegative::<Number>(Number::new(val.max(0.)))
	}

	pub fn parse<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		let value = Number::parse_non_negative(context, input)?;
		Ok(NonNegative::<Number>(value))
	}

	pub fn parse_in_range<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
		from: f32,
		to: f32,
	) -> Result<Self, ParseError<'i>> {
		let value = Number::parse_in_range(context, input, from, to)?;
		Ok(NonNegative::<Number>(value))
	}

	pub fn get(&self) -> f32 {
		self.0.get()
	}
}

impl PartialEq<i32> for NonNegativeNumber {
	fn eq(&self, other: &i32) -> bool {
		self.get() as i32 == *other
	}
}

impl PartialOrd<i32> for NonNegativeNumber {
	fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
		let value = self.get() as i32;
		if value > *other {
			Some(Ordering::Greater)
		} else if value < *other {
			Some(Ordering::Less)
		} else {
			Some(Ordering::Equal)
		}
	}
}

impl PartialEq<f32> for NonNegativeNumber {
	fn eq(&self, other: &f32) -> bool {
		self.get() == *other
	}
}

impl PartialOrd<f32> for NonNegativeNumber {
	fn partial_cmp(&self, other: &f32) -> Option<Ordering> {
		let value = self.get();
		if value > *other {
			Some(Ordering::Greater)
		} else if value < *other {
			Some(Ordering::Less)
		} else {
			Some(Ordering::Equal)
		}
	}
}

/// Generic for Number/Auto
#[derive(Clone)]
pub enum GenericNumberOrAuto<Number> {
	Number(Number),
	Auto,
}

impl<L> GenericNumberOrAuto<L> {
	pub fn parse_with<'i, 't, LP>(
		input: &mut Parser<'i, 't>,
		number_parser: LP,
	) -> Result<Self, ParseError<'i>>
	where
		LP: FnOnce(&mut Parser<'i, 't>) -> Result<L, ParseError<'i>>,
	{
		input
			.try_parse(|input| {
				input.expect_ident_matching("auto")?;
				Ok(Self::Auto)
			})
			.or_else(|_err: ParseError<'i>| {
				let length = number_parser(input)?;
				Ok(Self::Number(length))
			})
	}
}

impl<N: ToCss> ToCss for GenericNumberOrAuto<N> {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		match self {
			GenericNumberOrAuto::Number(value) => value.to_css(dest),
			GenericNumberOrAuto::Auto => dest.write_str("auto"),
		}
	}
}

pub type IntegerAuto = GenericNumberOrAuto<Integer>;

impl IntegerAuto {
	pub fn parse<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		Self::parse_with(input, |input| Integer::parse(context, input))
	}
}

pub type NumberOrPercentage = GenericNumberOrPercentage<Number>;

impl NumberOrPercentage {
	pub fn parse<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		Self::parse_with(context, input, |input| Number::parse(context, input))
	}

	pub fn parse_in_range<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
		number_range: &Range<f32>,
	) -> Result<Self, ParseError<'i>> {
		Self::parse_with(context, input, |input| {
			let value = Number::parse(context, input)?;
			if value.get() >= number_range.start && value.get() <= number_range.end {
				Ok(value)
			} else {
				Err(input.new_custom_error(StyleParseErrorKind::UnspecifiedError))
			}
		})
	}
}

impl From<&str> for NumberOrPercentage {
	fn from(text: &str) -> Self {
		if let Some(index) = text.find(|ch: char| ch == '%') {
			let value = text[..index].parse::<f32>().unwrap();
			Self::Percentage(Percentage::new(value / 100.0))
		} else {
			let value = text.parse::<f32>().unwrap();
			Self::Number(Number::new(value))
		}
	}
}

pub type NonNegativeNumberOrPercentage = GenericNumberOrPercentage<NonNegativeNumber>;

impl NonNegativeNumberOrPercentage {
	pub fn parse<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		Self::parse_with(context, input, |input| {
			NonNegativeNumber::parse(context, input)
		})
	}
}

impl From<&str> for NonNegativeNumberOrPercentage {
	fn from(text: &str) -> Self {
		if let Some(index) = text.find(|ch: char| ch == '%') {
			let value = text[..index].parse::<f32>().unwrap();
			Self::Percentage(Percentage::new(value))
		} else {
			let value = text.parse::<f32>().unwrap();
			assert!(value >= 0.0 && value <= 100.0);
			Self::Number(NonNegativeNumber::new(value))
		}
	}
}

pub struct Zero;

impl Zero {
	pub fn parse<'i, 't>(
		_context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		let value = input.expect_number()?;
		if value == 0.0 {
			Ok(Zero)
		} else {
			Err(input.new_custom_error(StyleParseErrorKind::UnspecifiedError))
		}
	}
}
