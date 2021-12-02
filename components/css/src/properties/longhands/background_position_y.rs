use cssparser::{match_ignore_ascii_case, Parser, ToCss, Token, _cssparser_internal_to_lowercase};

use crate::parser::ParseError;
use crate::properties::declaration::{property_keywords_impl, PropertyDeclaration};
use crate::str::convert_options_to_string;
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::specified::length::LengthPercentage;

#[derive(Clone, Debug)]
pub enum VerticalPositionKeyword {
	Top,
	Bottom,
	YStart,
	YEnd,
}

property_keywords_impl! { VerticalPositionKeyword,
	VerticalPositionKeyword::Top, "top",
	VerticalPositionKeyword::Bottom, "bottom",
	VerticalPositionKeyword::YStart, "y-start",
	VerticalPositionKeyword::YEnd, "y-end",
}

#[derive(Clone, Debug)]
pub struct VerticalPosition {
	keyword: Option<VerticalPositionKeyword>,
	length: Option<LengthPercentage>,
}

impl VerticalPosition {
	pub fn parse<'i, 't>(context: &ParserContext, input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i>> {
		let keyword_parser_ret = input.try_parse(|input| VerticalPositionKeyword::parse(input));
		let length_parser_ret = input.try_parse(|input| LengthPercentage::parse(context, input));

		if keyword_parser_ret.is_err() && length_parser_ret.is_err() {
			Err(input.new_custom_error(StyleParseErrorKind::UnspecifiedError))
		} else {
			Ok(VerticalPosition {
				keyword: keyword_parser_ret.ok(),
				length: length_parser_ret.ok(),
			})
		}
	}
}

impl ToCss for VerticalPosition {
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
pub enum VerticalPositionComponent {
	Center,
	PositionY(VerticalPosition),
}

impl VerticalPositionComponent {
	pub fn parse<'i, 't>(context: &ParserContext, input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i>> {
		input
			.try_parse(|input| {
				input.expect_ident_matching("center")?;
				Ok(VerticalPositionComponent::Center)
			})
			.or_else(|_err: ParseError<'i>| {
				let position = VerticalPosition::parse(context, input)?;
				Ok(VerticalPositionComponent::PositionY(position))
			})
	}
}

impl ToCss for VerticalPositionComponent {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		match self {
			VerticalPositionComponent::Center => dest.write_str("center"),
			VerticalPositionComponent::PositionY(value) => value.to_css(dest),
		}
	}
}

/// https://drafts.csswg.org/css-backgrounds-4/#propdef-background-position-y
#[derive(Clone, Debug)]
pub struct BackgroundPositionY {
	positions: Vec<VerticalPositionComponent>,
}

impl BackgroundPositionY {
	pub fn parse<'i, 't>(context: &ParserContext, input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i>> {
		let positions = input.parse_comma_separated(|input| VerticalPositionComponent::parse(context, input))?;
		Ok(BackgroundPositionY { positions })
	}
}

impl ToCss for BackgroundPositionY {
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
	BackgroundPositionY::parse(context, input).map(PropertyDeclaration::BackgroundPositionY)
}
