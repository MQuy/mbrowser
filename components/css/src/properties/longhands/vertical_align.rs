use cssparser::{match_ignore_ascii_case, Parser, ToCss, Token, _cssparser_internal_to_lowercase};

use crate::parser::ParseError;
use crate::properties::declaration::{property_keywords_impl, PropertyDeclaration};
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::specified::length::LengthPercentage;

#[derive(Clone, Debug)]
#[repr(u8)]
pub enum VerticalAlignKeyword {
	Baseline,
	Sub,
	Super,
	Top,
	TextTop,
	Middle,
	Bottom,
	TextBottom,
}

property_keywords_impl! { VerticalAlignKeyword,
	VerticalAlignKeyword::Baseline, "baseline",
	VerticalAlignKeyword::Sub, "sub",
	VerticalAlignKeyword::Super, "super",
	VerticalAlignKeyword::Top, "top",
	VerticalAlignKeyword::TextTop, "text-top",
	VerticalAlignKeyword::Middle, "middle",
	VerticalAlignKeyword::Bottom, "bottom",
	VerticalAlignKeyword::TextBottom, "text-bottom",
}

/// https://drafts.csswg.org/css2/#propdef-vertical-align
#[derive(Clone, Debug)]
pub enum VerticalAlign {
	Keyword(VerticalAlignKeyword),
	LengthPercentage(LengthPercentage),
}

impl VerticalAlign {
	pub fn parse<'i, 't>(context: &ParserContext, input: &mut Parser<'i, 't>) -> Result<VerticalAlign, ParseError<'i>> {
		input
			.try_parse(|input| {
				let keyword = VerticalAlignKeyword::parse(input)?;
				Ok(VerticalAlign::Keyword(keyword))
			})
			.or_else(|_err: ParseError<'i>| {
				let value = LengthPercentage::parse(context, input)?;
				Ok(VerticalAlign::LengthPercentage(value))
			})
	}
}

impl ToCss for VerticalAlign {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		match self {
			VerticalAlign::Keyword(value) => value.to_css(dest),
			VerticalAlign::LengthPercentage(value) => value.to_css(dest),
		}
	}
}

pub fn parse_declared<'i, 't>(
	context: &ParserContext,
	input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
	VerticalAlign::parse(context, input).map(PropertyDeclaration::VerticalAlign)
}
