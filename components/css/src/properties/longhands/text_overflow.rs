use cssparser::{Parser, ToCss, _cssparser_internal_to_lowercase, match_ignore_ascii_case};

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::specified::length::LengthPercentage;

#[derive(Clone, Debug)]
pub enum TextOverflowSide {
	Clip,
	Ellipsis,
	String(String),
	Fade(Option<LengthPercentage>),
}

impl TextOverflowSide {
	pub fn parse<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		input
			.try_parse(|input| {
				let location = input.current_source_location();
				let ident = input.expect_ident()?;
				Ok(match_ignore_ascii_case! { ident,
					"clip" => TextOverflowSide::Clip,
					"ellipsis" => TextOverflowSide::Ellipsis,
					"fade" => TextOverflowSide::Fade(None),
					_ => return Err(location.new_custom_error(StyleParseErrorKind::UnexpectedValue(ident.clone())))
				})
			})
			.or_else(|_err: ParseError<'i>| {
				let value = input.try_parse(|input| -> Result<String, ParseError<'i>> {
					let value = input.expect_string()?.to_string();
					Ok(value)
				})?;
				Ok(TextOverflowSide::String(value))
			})
			.or_else(|_err: ParseError<'i>| {
				input.expect_function_matching("fade")?;
				let arg =
					input.parse_nested_block(|input| LengthPercentage::parse(context, input))?;
				Ok(TextOverflowSide::Fade(Some(arg)))
			})
	}
}

impl ToCss for TextOverflowSide {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		match self {
			TextOverflowSide::Clip => dest.write_str("clip"),
			TextOverflowSide::Ellipsis => dest.write_str("ellipsis"),
			TextOverflowSide::String(value) => dest.write_fmt(std::format_args!("\"{}\"", value)),
			TextOverflowSide::Fade(value) => {
				if let Some(value) = value {
					dest.write_fmt(format_args!("fade({})", value.to_css_string()))
				} else {
					dest.write_str("fade")
				}
			},
		}
	}
}

/// https://drafts.csswg.org/css-overflow-4/#text-overflow
#[derive(Clone, Debug)]
pub struct TextOverflow {
	first: TextOverflowSide,
	second: Option<TextOverflowSide>,
}

impl TextOverflow {
	pub fn parse<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<TextOverflow, ParseError<'i>> {
		let first = TextOverflowSide::parse(context, input)?;
		let second = TextOverflowSide::parse(context, input).ok();
		Ok(TextOverflow { first, second })
	}
}

impl ToCss for TextOverflow {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		self.first.to_css(dest)?;
		if let Some(side) = &self.second {
			dest.write_char(' ')?;
			side.to_css(dest)?;
		}
		Ok(())
	}
}

pub fn parse_declared<'i, 't>(
	context: &ParserContext,
	input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
	TextOverflow::parse(context, input).map(PropertyDeclaration::TextOverflow)
}
