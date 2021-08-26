use cssparser::{Parser, ToCss, _cssparser_internal_to_lowercase, match_ignore_ascii_case};

use crate::parser::ParseError;
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::length::NonNegativeLength;

#[derive(Clone)]
pub enum LineWidth {
	Thin,
	Medium,
	Thick,
	Length(NonNegativeLength),
}

impl LineWidth {
	pub fn parse<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		input
			.try_parse(|input| {
				let location = input.current_source_location();
				let ident = input.expect_ident()?;
				Ok(match_ignore_ascii_case! { ident,
					"thin" => LineWidth::Thin,
					"medium" => LineWidth::Medium,
					"thick" => LineWidth::Thick,
					_ => return Err(location.new_custom_error(StyleParseErrorKind::UnexpectedValue(ident.clone())))
				})
			})
			.or_else(|_err: ParseError<'i>| {
				let length = NonNegativeLength::parse(context, input)?;
				Ok(LineWidth::Length(length))
			})
	}
}

impl ToCss for LineWidth {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		match self {
			LineWidth::Thin => dest.write_str("thin"),
			LineWidth::Medium => dest.write_str("medium"),
			LineWidth::Thick => dest.write_str("thick"),
			LineWidth::Length(length) => length.to_css(dest),
		}
	}
}
