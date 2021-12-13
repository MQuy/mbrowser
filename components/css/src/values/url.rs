use common::url::BrowserUrl;
use cssparser::{Parser, ToCss, Token, _cssparser_internal_to_lowercase, match_ignore_ascii_case};

use super::Ident;
use crate::media_queries::media_condition::consume_any_value;
use crate::parser::{parse_repeated, ParseError};
use crate::stylesheets::rule_parser::StyleParseErrorKind;

#[derive(Clone, Debug)]
pub enum UrlModifier {
	Ident(Ident),
	Function(String),
}

impl ToCss for UrlModifier {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		match self {
			UrlModifier::Ident(ident) => ident.to_css(dest),
			UrlModifier::Function(value) => dest.write_str(value),
		}
	}
}

/// https://drafts.csswg.org/css-values-4/#urls
#[derive(Clone, Debug)]
pub struct CssUrl {
	original: String,
	resolved: Option<BrowserUrl>,
	modifiers: Vec<UrlModifier>,
}

impl CssUrl {
	pub fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i>> {
		let location = input.current_source_location();
		let token = input.next()?.clone();
		let (name, value, modifiers) = match token {
			Token::Function(ref name) => input.parse_nested_block(|input| {
				let value = match_ignore_ascii_case! { name,
						"url" | "src" => Ok(input.expect_string()?.to_string()),
						_ => Err(location.new_custom_error(StyleParseErrorKind::UnexpectedValue(name.clone())))
				}?;
				let modifiers =
					parse_repeated(input, &mut |input| CssUrl::parse_url_modifier(input), 0).map_or(vec![], |v| v);
				Ok((name.to_string(), value, modifiers))
			})?,
			Token::UnquotedUrl(ref value) => ("url".to_string(), value.to_string(), vec![]),
			_ => return Err(location.new_custom_error(StyleParseErrorKind::UnexpectedToken(token.clone()))),
		};
		let url =
			BrowserUrl::parse(&value).map_err(|_err| input.new_custom_error(StyleParseErrorKind::UnspecifiedError))?;
		Ok(CssUrl {
			original: std::format!(
				"{}(\"{}\"{})",
				name,
				value,
				if modifiers.len() > 0 {
					std::format!(
						" {}",
						modifiers
							.iter()
							.map(|v| v.to_css_string())
							.collect::<Vec<String>>()
							.join(" ")
					)
				} else {
					"".to_string()
				}
			),
			resolved: Some(url),
			modifiers,
		})
	}

	pub fn parse_string<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i>> {
		let value = input.expect_string()?.to_string();
		Ok(CssUrl {
			original: std::format!("\"{}\"", value),
			resolved: None,
			modifiers: vec![],
		})
	}

	pub fn parse_url_modifier<'i, 't>(input: &mut Parser<'i, 't>) -> Result<UrlModifier, ParseError<'i>> {
		input
			.try_parse(|input| {
				input.skip_whitespace();
				let position = input.position();
				input.expect_function()?;
				input.parse_nested_block(|input| consume_any_value(input))?;
				Ok(UrlModifier::Function(input.slice_from(position).to_owned()))
			})
			.or_else(|_err: ParseError<'i>| {
				let ident = input.expect_ident()?.to_string();
				Ok(UrlModifier::Ident(Ident::new(ident)))
			})
	}
}

impl ToCss for CssUrl {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		dest.write_str(&self.original)
	}
}
