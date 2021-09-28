use cssparser::{match_ignore_ascii_case, Parser, ToCss, Token, _cssparser_internal_to_lowercase};

use super::length::LengthPercentage;
use crate::parser::{parse_in_any_order, parse_item_if_missing, ParseError};
use crate::properties::declaration::property_keywords_impl;
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;

macro_rules! return_unexpected_token {
	($location:tt, $token:tt) => {
		return Err($location.new_custom_error(StyleParseErrorKind::UnexpectedToken($token.clone())))
	};
}

#[derive(Clone, Debug)]
pub enum LeftOrRight {
	Left,
	Right,
}

property_keywords_impl! { LeftOrRight,
	LeftOrRight::Left, "left",
	LeftOrRight::Right, "right",
}

#[derive(Clone, Debug)]
pub enum HorizontalPosition {
	Left,
	Right,
	Center,
	Length(LengthPercentage),
	Side(LeftOrRight, Option<LengthPercentage>),
}

impl HorizontalPosition {
	pub fn parse<'i, 't>(
		_context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		let location = input.current_source_location();
		let token = input.next()?;
		Ok(match token {
			Token::Ident(value) => match_ignore_ascii_case! { value,
				"left" => HorizontalPosition::Left,
				"right" => HorizontalPosition::Right,
				"center" => HorizontalPosition::Center,
				_ => return_unexpected_token!(location, token),
			},
			_ => return_unexpected_token!(location, token),
		})
	}

	pub fn parse_with_length<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		input
			.try_parse(|input| HorizontalPosition::parse(context, input))
			.or_else(|_err: ParseError<'i>| {
				let length = LengthPercentage::parse(context, input)?;
				Ok(HorizontalPosition::Length(length))
			})
	}

	pub fn parse_side<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		input.try_parse(|input| {
			let keyword = LeftOrRight::parse(input)?;
			let length = LengthPercentage::parse(context, input).ok();
			Ok(HorizontalPosition::Side(keyword, length))
		})
	}
}

impl ToCss for HorizontalPosition {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		match self {
			HorizontalPosition::Left => dest.write_str("left"),
			HorizontalPosition::Right => dest.write_str("right"),
			HorizontalPosition::Center => dest.write_str("center"),
			HorizontalPosition::Length(length) => length.to_css(dest),
			HorizontalPosition::Side(side, value) => {
				side.to_css(dest)?;
				value.as_ref().map_or(Ok(()), |v| {
					dest.write_fmt(format_args!(" {}", v.to_css_string()))
				})
			},
		}
	}
}

#[derive(Clone, Debug)]
pub enum TopOrBottom {
	Top,
	Bottom,
}

property_keywords_impl! { TopOrBottom,
	TopOrBottom::Top, "top",
	TopOrBottom::Bottom, "bottom",
}

#[derive(Clone, Debug)]
pub enum VerticalPosition {
	Top,
	Bottom,
	Center,
	Length(LengthPercentage),
	Side(TopOrBottom, Option<LengthPercentage>),
}

impl VerticalPosition {
	pub fn parse<'i, 't>(
		_context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		let location = input.current_source_location();
		let token = input.next()?;
		Ok(match token {
			Token::Ident(value) => match_ignore_ascii_case! { value,
				"top" => VerticalPosition::Top,
				"bottom" => VerticalPosition::Bottom,
				"center" => VerticalPosition::Center,
				_ => return_unexpected_token!(location, token),
			},
			_ => return_unexpected_token!(location, token),
		})
	}

	pub fn parse_with_length<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		input
			.try_parse(|input| VerticalPosition::parse(context, input))
			.or_else(|_err: ParseError<'i>| {
				let length = LengthPercentage::parse(context, input)?;
				Ok(VerticalPosition::Length(length))
			})
	}

	pub fn parse_side<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		input.try_parse(|input| {
			let keyword = TopOrBottom::parse(input)?;
			let length = LengthPercentage::parse(context, input).ok();
			Ok(VerticalPosition::Side(keyword, length))
		})
	}
}

impl ToCss for VerticalPosition {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		match self {
			VerticalPosition::Top => dest.write_str("top"),
			VerticalPosition::Bottom => dest.write_str("bottom"),
			VerticalPosition::Center => dest.write_str("center"),
			VerticalPosition::Length(value) => value.to_css(dest),
			VerticalPosition::Side(side, value) => {
				side.to_css(dest)?;
				value.as_ref().map_or(Ok(()), |v| {
					dest.write_fmt(format_args!(" {}", v.to_css_string()))
				})
			},
		}
	}
}

/// https://drafts.csswg.org/css-values-4/#position
#[derive(Clone, Debug)]
pub struct Position {
	horizontal: HorizontalPosition,
	vertical: VerticalPosition,
}

impl Position {
	pub fn new(horizontal: HorizontalPosition, vertical: VerticalPosition) -> Self {
		Self {
			horizontal,
			vertical,
		}
	}

	pub fn parse<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		input
			.try_parse(|input| {
				let horizontal = HorizontalPosition::parse_side(context, input)?;
				let vertical = VerticalPosition::parse_side(context, input)?;
				Ok(Position {
					horizontal,
					vertical,
				})
			})
			.or_else(|_err: ParseError<'i>| {
				input.try_parse(|input| {
					let horizontal = HorizontalPosition::parse_with_length(context, input)?;
					let vertical = VerticalPosition::parse_with_length(context, input)
						.map_or(VerticalPosition::Center, |v| v);
					Ok(Position {
						horizontal,
						vertical,
					})
				})
			})
			.or_else(|_err: ParseError<'i>| {
				let mut horizontal = None;
				let mut vertical = None;
				parse_in_any_order(
					input,
					&mut [
						&mut |input| {
							parse_item_if_missing(input, &mut horizontal, &mut |_, input| {
								HorizontalPosition::parse(context, input)
							})
						},
						&mut |input| {
							parse_item_if_missing(input, &mut vertical, &mut |_, input| {
								VerticalPosition::parse(context, input)
							})
						},
					],
				);
				if horizontal.is_none() && vertical.is_none() {
					Err(input.new_custom_error(StyleParseErrorKind::UnspecifiedError))
				} else {
					Ok(Position {
						horizontal: horizontal.map_or(HorizontalPosition::Center, |v| v),
						vertical: vertical.map_or(VerticalPosition::Center, |v| v),
					})
				}
			})
	}
}

impl ToCss for Position {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		self.horizontal.to_css(dest)?;
		dest.write_char(' ')?;
		self.vertical.to_css(dest)
	}
}

#[derive(Clone, Debug)]
pub enum FirstOrLast {
	First,
	Last,
}

property_keywords_impl! { FirstOrLast,
	FirstOrLast::First, "first",
	FirstOrLast::Last, "last",
}

#[derive(Clone, Debug)]
pub struct BaselinePosition {
	preference: FirstOrLast,
}

impl BaselinePosition {
	pub fn parse<'i, 't>(
		_context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		let preference = input
			.try_parse(|input| FirstOrLast::parse(input))
			.map_or(FirstOrLast::First, |v| v);
		input.expect_ident_matching("baseline")?;
		Ok(BaselinePosition { preference })
	}
}

impl ToCss for BaselinePosition {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		self.preference.to_css(dest)?;
		dest.write_str(" baseline")
	}
}

#[derive(Clone, Debug)]
pub enum ContentDistribution {
	SpaceBetween,
	SpaceAround,
	SpaceEvenly,
	Stretch,
}

property_keywords_impl! { ContentDistribution,
	ContentDistribution::SpaceBetween, "space-between",
	ContentDistribution::SpaceAround, "space-around",
	ContentDistribution::SpaceEvenly, "space-evenly",
	ContentDistribution::Stretch, "stretch",
}

#[derive(Clone, Debug)]
pub enum OverflowPosition {
	Unsafe,
	Safe,
}

property_keywords_impl! { OverflowPosition,
	OverflowPosition::Unsafe, "unsafe",
	OverflowPosition::Safe, "safe",
}

#[derive(Clone, Debug)]
pub enum ContentPosition {
	Center,
	Start,
	End,
	FlexStart,
	FlexEnd,
}

property_keywords_impl! { ContentPosition,
	ContentPosition::Center, "center",
	ContentPosition::Start, "start",
	ContentPosition::End, "end",
	ContentPosition::FlexStart, "flex-start",
	ContentPosition::FlexEnd, "flex-end",
}
#[derive(Clone, Debug)]
pub enum SelfPosition {
	Center,
	Start,
	End,
	SelfStart,
	SelfEnd,
	FlexStart,
	FlexEnd,
}

property_keywords_impl! { SelfPosition,
	SelfPosition::Center, "center",
	SelfPosition::Start, "start",
	SelfPosition::End, "end",
	SelfPosition::SelfStart, "self-start",
	SelfPosition::SelfEnd, "self-end",
	SelfPosition::FlexStart, "flex-start",
	SelfPosition::FlexEnd, "flex-end",
}
