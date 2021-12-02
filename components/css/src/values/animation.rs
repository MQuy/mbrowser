use cssparser::{Parser, ToCss, Token};

use super::CustomIdent;
use crate::parser::ParseError;
use crate::stylesheets::stylesheet::ParserContext;

#[derive(Clone, Debug)]
pub enum KeyframesName {
	Ident(CustomIdent),
	QuotedString(String),
}

impl KeyframesName {
	pub fn parse<'i, 't>(_context: &ParserContext, input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i>> {
		let location = input.current_source_location();
		match input.next()? {
			Token::Ident(ref ident) => Ok(KeyframesName::Ident(CustomIdent::from_ident(
				location,
				ident,
				&["none"],
			)?)),
			Token::QuotedString(ref value) => Ok(KeyframesName::QuotedString(value.to_string())),
			t => Err(location.new_unexpected_token_error(t.clone())),
		}
	}
}

impl ToCss for KeyframesName {
	fn to_css<W>(&self, dest: &mut W) -> core::fmt::Result
	where
		W: std::fmt::Write,
	{
		match self {
			KeyframesName::Ident(ident) => ident.to_css(dest),
			KeyframesName::QuotedString(value) => dest.write_fmt(format_args!("\"{}\"", value)),
		}
	}
}
