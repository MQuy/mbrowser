use cssparser::{Parser, ToCss, _cssparser_internal_to_lowercase, match_ignore_ascii_case};

use crate::parser::{parse_repeated, ParseError};
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::specified::counter::Counter;
use crate::values::specified::image::Image;
use crate::values::specified::leader::Leader;
use crate::values::specified::quote::Quote;
use crate::values::specified::target::Target;

#[derive(Clone, Debug)]
pub enum ContentList {
	String(String),
	Contents,
	Image(Image),
	Counter(Counter),
	Quote(Quote),
	Target(Target),
	Leader(Leader),
}

impl ContentList {
	pub fn parse<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		input
			.try_parse(|input| {
				input.expect_ident_matching("contents")?;
				Ok(ContentList::Contents)
			})
			.or_else(|_err: ParseError<'i>| {
				input.try_parse(|input| {
					let value = input.expect_string()?.to_string();
					Ok(ContentList::String(value))
				})
			})
			.or_else(|_err: ParseError<'i>| {
				input.try_parse(|input| {
					let image = Image::parse(context, input)?;
					Ok(ContentList::Image(image))
				})
			})
			.or_else(|_err: ParseError<'i>| {
				input.try_parse(|input| {
					let counter = Counter::parse(context, input)?;
					Ok(ContentList::Counter(counter))
				})
			})
			.or_else(|_err: ParseError<'i>| {
				input.try_parse(|input| {
					let quote = Quote::parse(input)?;
					Ok(ContentList::Quote(quote))
				})
			})
			.or_else(|_err: ParseError<'i>| {
				input.try_parse(|input| {
					let target = Target::parse(context, input)?;
					Ok(ContentList::Target(target))
				})
			})
			.or_else(|_err: ParseError<'i>| {
				input.try_parse(|input| {
					let leader = Leader::parse(context, input)?;
					Ok(ContentList::Leader(leader))
				})
			})
	}
}

impl ToCss for ContentList {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		match self {
			ContentList::String(value) => dest.write_fmt(std::format_args!("\"{}\"", value)),
			ContentList::Contents => dest.write_str("contents"),
			ContentList::Image(value) => value.to_css(dest),
			ContentList::Counter(value) => value.to_css(dest),
			ContentList::Quote(value) => value.to_css(dest),
			ContentList::Target(value) => value.to_css(dest),
			ContentList::Leader(value) => value.to_css(dest),
		}
	}
}

#[derive(Clone, Debug)]
pub enum ContentReplacementOrList {
	Replacement(Image),
	List(Vec<ContentList>),
}

impl ContentReplacementOrList {
	pub fn parse<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		input
			.try_parse(|input| {
				let image = Image::parse(context, input)?;
				Ok(ContentReplacementOrList::Replacement(image))
			})
			.or_else(|_err: ParseError<'i>| {
				let values =
					parse_repeated(input, &mut |input| ContentList::parse(context, input), 1)?;
				Ok(ContentReplacementOrList::List(values))
			})
	}
}

impl ToCss for ContentReplacementOrList {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		match self {
			ContentReplacementOrList::Replacement(value) => value.to_css(dest),
			ContentReplacementOrList::List(value) => dest.write_str(
				&value
					.iter()
					.map(|v| v.to_css_string())
					.collect::<Vec<String>>()
					.join(" "),
			),
		}
	}
}

#[derive(Clone, Debug)]
pub enum CounterOrString {
	Counter(Counter),
	String(String),
}

impl CounterOrString {
	pub fn parse<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		input
			.try_parse(|input| {
				let value = Counter::parse(context, input)?;
				Ok(CounterOrString::Counter(value))
			})
			.or_else(|_err: ParseError<'i>| {
				let value = input.expect_string()?.to_string();
				Ok(CounterOrString::String(value))
			})
	}
}

impl ToCss for CounterOrString {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		match self {
			CounterOrString::Counter(value) => value.to_css(dest),
			CounterOrString::String(value) => dest.write_fmt(std::format_args!("\"{}\"", value)),
		}
	}
}

#[derive(Clone, Debug)]
pub struct ContentData {
	content: ContentReplacementOrList,
	alt: Vec<CounterOrString>,
}

impl ContentData {
	pub fn parse<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		let content = ContentReplacementOrList::parse(context, input)?;
		let alt = input
			.try_parse(|input| -> Result<Vec<CounterOrString>, ParseError<'i>> {
				input.expect_delim('/')?;
				let value = parse_repeated(
					input,
					&mut |input| CounterOrString::parse(context, input),
					1,
				)?;
				Ok(value)
			})
			.map_or(vec![], |alt| alt);
		Ok(ContentData { content, alt })
	}
}

impl ToCss for ContentData {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		self.content.to_css(dest)?;
		if self.alt.len() > 0 {
			dest.write_str(" / ")?;
			dest.write_str(
				&self
					.alt
					.iter()
					.map(|v| v.to_css_string())
					.collect::<Vec<String>>()
					.join(" "),
			)?;
		}
		Ok(())
	}
}

/// https://drafts.csswg.org/css-content/#content-property
#[derive(Clone, Debug)]
pub enum Content {
	Normal,
	None,
	Data(ContentData),
}

impl Content {
	pub fn parse<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		input
			.try_parse(|input| {
				let location = input.current_source_location();
				let ident = input.expect_ident()?;
				Ok(match_ignore_ascii_case! { ident,
					"normal" => Content::Normal,
					"none" => Content::None,
					_ => return Err(location.new_custom_error(StyleParseErrorKind::UnexpectedValue(ident.clone())))
				})
			})
			.or_else(|_err: ParseError<'i>| {
				let value = ContentData::parse(context, input)?;
				Ok(Content::Data(value))
			})
	}
}

impl ToCss for Content {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		match self {
			Content::Normal => dest.write_str("normal"),
			Content::None => dest.write_str("none"),
			Content::Data(value) => value.to_css(dest),
		}
	}
}

pub fn parse_declared<'i, 't>(
	context: &ParserContext,
	input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
	Content::parse(context, input).map(PropertyDeclaration::Content)
}
