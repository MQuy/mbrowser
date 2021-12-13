use cssparser::{Parser, ToCss};

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::specified::counter::CounterStyle;

#[derive(Clone, Debug)]
pub enum ListStyleType {
	None,
	String(String),
	Style(CounterStyle),
}

impl ListStyleType {
	pub fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i>> {
		input
			.try_parse(|input| {
				input.expect_ident_matching("none")?;
				Ok(ListStyleType::None)
			})
			.or_else(|_err: ParseError<'i>| {
				input.try_parse(|input| {
					let value = input.expect_string()?.to_string();
					Ok(ListStyleType::String(value))
				})
			})
			.or_else(|_err: ParseError<'i>| {
				let style = CounterStyle::parse(input)?;
				Ok(ListStyleType::Style(style))
			})
	}
}

impl ToCss for ListStyleType {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		match self {
			ListStyleType::None => dest.write_str("none"),
			ListStyleType::String(value) => dest.write_fmt(std::format_args!("\"{}\"", value)),
			ListStyleType::Style(value) => value.to_css(dest),
		}
	}
}

pub fn parse_declared<'i, 't>(
	_context: &ParserContext,
	input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
	ListStyleType::parse(input).map(PropertyDeclaration::ListStyleType)
}
