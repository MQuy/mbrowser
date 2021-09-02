use cssparser::{Parser, ToCss};

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::layout::LineStyle;

/// https://drafts.csswg.org/css-ui/#outline-style
#[derive(Clone)]
pub enum OutlineStyle {
	Auto,
	BorderStyle(LineStyle),
}

impl OutlineStyle {
	pub fn parse<'i, 't>(
		_context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<OutlineStyle, ParseError<'i>> {
		input
			.try_parse(|input| {
				input.expect_ident_matching("auto")?;
				Ok(OutlineStyle::Auto)
			})
			.or_else(|_err: ParseError<'i>| {
				let style = LineStyle::parse(input)?;
				if style == LineStyle::Hidden {
					return Err(input
						.new_custom_error(StyleParseErrorKind::UnexpectedValue("hidden".into())));
				}
				Ok(OutlineStyle::BorderStyle(style))
			})
	}
}

impl ToCss for OutlineStyle {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		match self {
			OutlineStyle::Auto => dest.write_str("auto"),
			OutlineStyle::BorderStyle(value) => value.to_css(dest),
		}
	}
}

pub fn parse_declared<'i, 't>(
	context: &ParserContext,
	input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
	OutlineStyle::parse(context, input).map(PropertyDeclaration::OutlineStyle)
}
