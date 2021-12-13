use cssparser::{match_ignore_ascii_case, Parser, ToCss, Token, _cssparser_internal_to_lowercase};

use super::image::Image;
use super::number::Integer;
use crate::parser::{parse_repeated, ParseError};
use crate::properties::declaration::property_keywords_impl;
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::generics::counter::{GenericCounter, GenericCounterOrNone};
use crate::values::CustomIdent;

pub type CounterWithInteger = GenericCounterOrNone<GenericCounter<Integer>>;

impl CounterWithInteger {
	pub fn parse<'i, 't>(context: &ParserContext, input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i>> {
		GenericCounterOrNone::parse_with(input, |input| {
			GenericCounter::parse_with(input, |input| Integer::parse(context, input))
		})
	}
}

#[derive(Clone, Debug)]
pub enum SymbolsType {
	Cyclic,
	Numeric,
	Alphabetic,
	Symbolic,
	Fixed,
}

property_keywords_impl! { SymbolsType,
	SymbolsType::Cyclic, "cyclic",
	SymbolsType::Numeric, "numeric",
	SymbolsType::Alphabetic, "alphabetic",
	SymbolsType::Symbolic, "symbolic",
	SymbolsType::Fixed, "fixed",
}

#[derive(Clone, Debug)]
pub enum StringOrImage {
	String(String),
	Image(Image),
}

impl StringOrImage {
	pub fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i>> {
		input
			.try_parse(|input| {
				let value = input.expect_string()?.to_string();
				Ok(StringOrImage::String(value))
			})
			.or_else(|_err: ParseError<'i>| {
				let image = Image::parse(input)?;
				Ok(StringOrImage::Image(image))
			})
	}
}

impl ToCss for StringOrImage {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		match self {
			StringOrImage::String(value) => dest.write_fmt(std::format_args!("\"{}\"", value)),
			StringOrImage::Image(value) => value.to_css(dest),
		}
	}
}

#[derive(Clone, Debug)]
pub struct Symbols {
	symbols_type: Option<SymbolsType>,
	idents: Vec<StringOrImage>,
}

impl Symbols {
	pub fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i>> {
		input.expect_function_matching("symbols")?;
		input.parse_nested_block(|input| {
			let symbols_type = input.try_parse(|input| SymbolsType::parse(input)).ok();
			let idents = parse_repeated(input, &mut |input| StringOrImage::parse(input), 1)?;
			Ok(Symbols { symbols_type, idents })
		})
	}
}

impl ToCss for Symbols {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		dest.write_fmt(format_args!(
			"symbols({}{})",
			self.symbols_type
				.as_ref()
				.map_or("".to_string(), |v| std::format!("{} ", v.to_css_string())),
			self.idents
				.iter()
				.map(|v| v.to_css_string())
				.collect::<Vec<String>>()
				.join(" ")
		))
	}
}

#[derive(Clone, Debug)]
pub enum CounterStyle {
	Name(CustomIdent),
	Symbols(Symbols),
}

impl CounterStyle {
	pub fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i>> {
		input
			.try_parse(|input| {
				let name = CustomIdent::parse(input)?;
				Ok(CounterStyle::Name(name))
			})
			.or_else(|_err: ParseError<'i>| {
				let symbols = Symbols::parse(input)?;
				Ok(CounterStyle::Symbols(symbols))
			})
	}
}

impl ToCss for CounterStyle {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		match self {
			CounterStyle::Name(value) => value.to_css(dest),
			CounterStyle::Symbols(value) => value.to_css(dest),
		}
	}
}

#[derive(Clone, Debug)]
pub struct InnerMostCounter {
	name: CustomIdent,
	style: Option<CounterStyle>,
}

impl InnerMostCounter {
	pub fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i>> {
		let name = CustomIdent::parse_excluding(input, &["none"])?;
		let style = input
			.try_parse(|input| {
				input.expect_comma()?;
				CounterStyle::parse(input)
			})
			.ok();
		Ok(InnerMostCounter { name, style })
	}
}

impl ToCss for InnerMostCounter {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		dest.write_fmt(format_args!(
			"counter({}{})",
			self.name.to_css_string(),
			self.style
				.as_ref()
				.map_or(String::from(""), |v| std::format!(", {}", v.to_css_string()))
		))
	}
}

#[derive(Clone, Debug)]
pub struct AllCounters {
	name: CustomIdent,
	string: String,
	style: Option<CounterStyle>,
}

impl AllCounters {
	pub fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i>> {
		let name = CustomIdent::parse_excluding(input, &["none"])?;
		input.expect_comma()?;
		let str = input.expect_string()?.to_string();
		let style = input
			.try_parse(|input| {
				input.expect_comma()?;
				CounterStyle::parse(input)
			})
			.ok();
		Ok(AllCounters {
			name,
			string: str,
			style,
		})
	}
}

impl ToCss for AllCounters {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		dest.write_fmt(format_args!(
			"counters({}, {}{})",
			self.name.to_css_string(),
			std::format!("\"{}\"", self.string),
			self.style
				.as_ref()
				.map_or(String::from(""), |v| std::format!(", {}", v.to_css_string()))
		))
	}
}

/// https://drafts.csswg.org/css-lists-3/#typedef-counter
#[derive(Clone, Debug)]
pub enum Counter {
	Counter(InnerMostCounter),
	Counters(AllCounters),
}

impl Counter {
	pub fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i>> {
		let location = input.current_source_location();
		let name = input.expect_function()?.clone();
		input.parse_nested_block(|input| {
			Ok(match_ignore_ascii_case! { &name,
				"counter" => Counter::Counter(InnerMostCounter::parse(input)?),
				"counters" => Counter::Counters(AllCounters::parse(input)?),
				_ => return Err(location.new_custom_error(StyleParseErrorKind::UnexpectedValue(name.clone())))
			})
		})
	}
}

impl ToCss for Counter {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		match self {
			Counter::Counter(value) => value.to_css(dest),
			Counter::Counters(value) => value.to_css(dest),
		}
	}
}
