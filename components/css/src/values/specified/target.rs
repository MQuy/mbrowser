use cssparser::{match_ignore_ascii_case, Parser, ToCss, Token, _cssparser_internal_to_lowercase};

use super::counter::CounterStyle;
use crate::parser::ParseError;
use crate::properties::declaration::property_keywords_impl;
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::url::CssUrl;
use crate::values::CustomIdent;

#[derive(Clone, Debug)]
pub struct TargetCounter {
	url: CssUrl,
	ident: CustomIdent,
	style: Option<CounterStyle>,
}

impl TargetCounter {
	pub fn parse<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		let url = input
			.try_parse(|input| CssUrl::parse(context, input))
			.or_else(|_err: ParseError<'i>| CssUrl::parse_string(context, input))?;
		input.expect_comma()?;
		let ident = CustomIdent::parse(input)?;
		let style = input
			.try_parse(|input| {
				input.expect_comma()?;
				CounterStyle::parse(context, input)
			})
			.ok();
		Ok(TargetCounter { url, ident, style })
	}
}

impl ToCss for TargetCounter {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		dest.write_fmt(format_args!(
			"target-counter({}, {}{})",
			self.url.to_css_string(),
			self.ident.to_css_string(),
			self.style
				.as_ref()
				.map_or("".to_string(), |v| std::format!(", {}", v.to_css_string()))
		))
	}
}

#[derive(Clone, Debug)]
pub struct TargetCounters {
	url: CssUrl,
	ident: CustomIdent,
	string: String,
	style: Option<CounterStyle>,
}

impl TargetCounters {
	pub fn parse<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		let url = input
			.try_parse(|input| CssUrl::parse(context, input))
			.or_else(|_err: ParseError<'i>| CssUrl::parse_string(context, input))?;
		input.expect_comma()?;
		let ident = CustomIdent::parse(input)?;
		input.expect_comma()?;
		let str = input.expect_string()?.to_string();
		let style = input
			.try_parse(|input| {
				input.expect_comma()?;
				CounterStyle::parse(context, input)
			})
			.ok();
		Ok(TargetCounters {
			url,
			ident,
			string: str,
			style,
		})
	}
}

impl ToCss for TargetCounters {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		dest.write_fmt(format_args!(
			"target-counters({}, {}, {}{})",
			self.url.to_css_string(),
			self.ident.to_css_string(),
			std::format!("\"{}\"", self.string),
			self.style
				.as_ref()
				.map_or("".to_string(), |v| std::format!(", {}", v.to_css_string()))
		))
	}
}

#[derive(Clone, Debug)]
pub enum TargetTextKeyword {
	Content,
	Before,
	After,
	FirstLetter,
}

property_keywords_impl! { TargetTextKeyword,
	TargetTextKeyword::Content, "content",
	TargetTextKeyword::Before, "before",
	TargetTextKeyword::After, "after",
	TargetTextKeyword::FirstLetter, "first-letter",
}

#[derive(Clone, Debug)]
pub struct TargetText {
	url: CssUrl,
	keyword: Option<TargetTextKeyword>,
}

impl TargetText {
	pub fn parse<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		let url = input
			.try_parse(|input| CssUrl::parse(context, input))
			.or_else(|_err: ParseError<'i>| CssUrl::parse_string(context, input))?;
		let keyword = input
			.try_parse(|input| {
				input.expect_comma()?;
				TargetTextKeyword::parse(input)
			})
			.ok();
		Ok(TargetText { url, keyword })
	}
}

impl ToCss for TargetText {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		dest.write_fmt(format_args!(
			"target-text({}{})",
			self.url.to_css_string(),
			self.keyword
				.as_ref()
				.map_or("".to_string(), |v| std::format!(", {}", v.to_css_string()))
		))
	}
}

#[derive(Clone, Debug)]
pub enum Target {
	Counter(TargetCounter),
	Counters(TargetCounters),
	Text(TargetText),
}

impl Target {
	/// https://drafts.csswg.org/css-content/#typedef-target
	pub fn parse<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		let location = input.current_source_location();
		let name = input.expect_function()?.clone();
		input.parse_nested_block(|input| {
			match_ignore_ascii_case! { &name,
				"target-counter" => Target::parse_counter(context, input),
				"target-counters" => Target::parse_counters(context, input),
				"target-text" => Target::parse_text(context, input),
				_ => return Err(location.new_custom_error(StyleParseErrorKind::UnexpectedValue(name.clone())))
			}
		})
	}

	fn parse_counter<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		let counter = TargetCounter::parse(context, input)?;
		Ok(Target::Counter(counter))
	}

	fn parse_counters<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		let counters = TargetCounters::parse(context, input)?;
		Ok(Target::Counters(counters))
	}

	fn parse_text<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		let text = TargetText::parse(context, input)?;
		Ok(Target::Text(text))
	}
}

impl ToCss for Target {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		match self {
			Target::Counter(value) => value.to_css(dest),
			Target::Counters(value) => value.to_css(dest),
			Target::Text(value) => value.to_css(dest),
		}
	}
}
