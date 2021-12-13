use cssparser::{Parser, ToCss};

use crate::parser::ParseError;
use crate::values::specified::percentage::Percentage;

#[derive(Clone, Debug, PartialEq)]
pub enum GenericNumberOrPercentage<Number> {
	Number(Number),
	Percentage(Percentage),
}

impl<N> GenericNumberOrPercentage<N> {
	pub fn parse_with<'i, 't, NP>(input: &mut Parser<'i, 't>, number_parser: NP) -> Result<Self, ParseError<'i>>
	where
		NP: FnOnce(&mut Parser<'i, 't>) -> Result<N, ParseError<'i>>,
	{
		input
			.try_parse(|input| {
				let value = Percentage::parse(input)?;
				Ok(Self::Percentage(value))
			})
			.or_else(|_err: ParseError<'i>| {
				let value = number_parser(input)?;
				Ok(Self::Number(value))
			})
	}
}

impl<N: ToCss> ToCss for GenericNumberOrPercentage<N> {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		match self {
			GenericNumberOrPercentage::Number(value) => value.to_css(dest),
			GenericNumberOrPercentage::Percentage(value) => value.to_css(dest),
		}
	}
}

#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub struct NonNegative<T>(pub T);

impl<T: ToCss + Clone> ToCss for NonNegative<T> {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		self.0.to_css(dest)
	}
}

#[derive(Clone, PartialEq, PartialOrd)]
pub struct GreaterThanOrEqualToOne<T>(pub T);

impl<T: ToCss> ToCss for GreaterThanOrEqualToOne<T> {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		self.0.to_css(dest)
	}
}

/// Generic for Number/Auto
#[derive(Clone, Debug)]
pub enum GenericNumberOrAuto<Number> {
	Number(Number),
	Auto,
}

impl<L> GenericNumberOrAuto<L> {
	pub fn parse_with<'i, 't, LP>(input: &mut Parser<'i, 't>, number_parser: LP) -> Result<Self, ParseError<'i>>
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
