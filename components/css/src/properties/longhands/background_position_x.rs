use cssparser::{match_ignore_ascii_case, Parser, ToCss, Token, _cssparser_internal_to_lowercase};

use crate::parser::ParseError;
use crate::properties::declaration::{property_keywords_impl, PropertyDeclaration};
use crate::str::convert_options_to_string;
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::length::LengthPercentage;

#[derive(Clone, Debug)]
pub enum HorizontalPositionKeyword {
	Left,
	Right,
	XStart,
	XEnd,
}

property_keywords_impl! { HorizontalPositionKeyword,
	HorizontalPositionKeyword::Left, "left",
	HorizontalPositionKeyword::Right, "right",
	HorizontalPositionKeyword::XStart, "x-start",
	HorizontalPositionKeyword::XEnd, "x-end",
}

#[derive(Clone, Debug)]
pub struct HorizontalPosition {
	keyword: Option<HorizontalPositionKeyword>,
	length: Option<LengthPercentage>,
}

impl HorizontalPosition {
	pub fn parse<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		let keyword_parser_ret = input.try_parse(|input| HorizontalPositionKeyword::parse(input));
		let length_parser_ret = input.try_parse(|input| LengthPercentage::parse(context, input));

		if keyword_parser_ret.is_err() && length_parser_ret.is_err() {
			Err(input.new_custom_error(StyleParseErrorKind::UnspecifiedError))
		} else {
			Ok(HorizontalPosition {
				keyword: keyword_parser_ret.ok(),
				length: length_parser_ret.ok(),
			})
		}
	}
}

impl ToCss for HorizontalPosition {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		let keyword = self.keyword.as_ref().map(|v| v.to_css_string());
		let length = self.length.as_ref().map(|v| v.to_css_string());
		dest.write_str(&convert_options_to_string(vec![keyword, length], " "))
	}
}

#[derive(Clone, Debug)]
pub enum HorizontalPositionComponent {
	Center,
	PositionX(HorizontalPosition),
}

impl ToCss for HorizontalPositionComponent {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		match self {
			HorizontalPositionComponent::Center => dest.write_str("center"),
			HorizontalPositionComponent::PositionX(value) => value.to_css(dest),
		}
	}
}

impl HorizontalPositionComponent {
	pub fn parse<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		input
			.try_parse(|input| {
				input.expect_ident_matching("center")?;
				Ok(HorizontalPositionComponent::Center)
			})
			.or_else(|_err: ParseError<'i>| {
				let position = HorizontalPosition::parse(context, input)?;
				Ok(HorizontalPositionComponent::PositionX(position))
			})
	}
}

/// https://drafts.csswg.org/css-backgrounds-4/#propdef-background-position-x
#[derive(Clone, Debug)]
pub struct BackgroundPositionX {
	positions: Vec<HorizontalPositionComponent>,
}

impl BackgroundPositionX {
	pub fn parse<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		let positions = input
			.parse_comma_separated(|input| HorizontalPositionComponent::parse(context, input))?;
		Ok(BackgroundPositionX { positions })
	}
}

impl ToCss for BackgroundPositionX {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		let values: Vec<String> = self.positions.iter().map(|v| v.to_css_string()).collect();
		dest.write_str(&values.join(", "))
	}
}

pub fn parse_declared<'i, 't>(
	context: &ParserContext,
	input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
	BackgroundPositionX::parse(context, input).map(PropertyDeclaration::BackgroundPositionX)
}
