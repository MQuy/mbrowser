use cssparser::{Parser, ToCss, Token, _cssparser_internal_to_lowercase, match_ignore_ascii_case};

use crate::parser::ParseError;
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;

#[derive(Clone, Debug)]
pub enum LeaderType {
	Dotted,
	Solid,
	Space,
	String(String),
}

impl LeaderType {
	pub fn parse<'i, 't>(
		_context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		let location = input.current_source_location();
		let token = input.next()?;
		Ok(match token {
			Token::Ident(ident) => match_ignore_ascii_case! { ident,
				"dotted" => LeaderType::Dotted,
				"solid" => LeaderType::Solid,
				"space" => LeaderType::Space,
				_ => return Err(location.new_custom_error(StyleParseErrorKind::UnexpectedValue(ident.clone())))
			},
			Token::QuotedString(text) => LeaderType::String(text.to_string()),
			_ => {
				return Err(
					location.new_custom_error(StyleParseErrorKind::UnexpectedToken(token.clone()))
				)
			},
		})
	}
}

impl ToCss for LeaderType {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		match self {
			LeaderType::Dotted => dest.write_str("dotted"),
			LeaderType::Solid => dest.write_str("solid"),
			LeaderType::Space => dest.write_str("space"),
			LeaderType::String(value) => dest.write_fmt(std::format_args!("\"{}\"", value)),
		}
	}
}

/// https://drafts.csswg.org/css-content/#leader-function
#[derive(Clone, Debug)]
pub struct Leader(LeaderType);

impl Leader {
	pub fn parse<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		input.expect_function_matching("leader")?;
		input.parse_nested_block(|input| {
			let style = LeaderType::parse(context, input)?;
			Ok(Leader(style))
		})
	}
}

impl ToCss for Leader {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		dest.write_fmt(format_args!("leader({})", self.0.to_css_string()))
	}
}
