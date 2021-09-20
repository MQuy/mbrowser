use cssparser::{Parser, ToCss, _cssparser_internal_to_lowercase, match_ignore_ascii_case};

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::length::LengthPercentageOrAuto;

#[derive(Clone, Debug)]
pub enum BgSize {
	ExplicitSize {
		width: LengthPercentageOrAuto,
		height: LengthPercentageOrAuto,
	},
	Cover,
	Contain,
}

impl BgSize {
	pub fn parse<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		input
			.try_parse(|input| {
				let width = LengthPercentageOrAuto::parse(context, input)?;
				let height = LengthPercentageOrAuto::parse(context, input);

				Ok(if let Ok(height) = height {
					BgSize::ExplicitSize { width, height }
				} else {
					BgSize::ExplicitSize {
						width,
						height: LengthPercentageOrAuto::Auto,
					}
				})
			})
			.or_else(|_err: ParseError<'i>| {
				let location = input.current_source_location();
				let ident = input.expect_ident()?;
				Ok(match_ignore_ascii_case! { ident,
					"cover" => BgSize::Cover,
					"contain" => BgSize::Contain,
					_ => return Err(location.new_custom_error(StyleParseErrorKind::UnexpectedValue(ident.clone())))
				})
			})
	}
}

impl ToCss for BgSize {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		match self {
			BgSize::ExplicitSize { width, height } => {
				width.to_css(dest)?;
				dest.write_char(' ')?;
				height.to_css(dest)
			},
			BgSize::Cover => dest.write_str("cover"),
			BgSize::Contain => dest.write_str("contain"),
		}
	}
}

/// https://drafts.csswg.org/css-backgrounds/#background-size
#[derive(Clone, Debug)]
pub struct BackgroundSize {
	size: Vec<BgSize>,
}

impl BackgroundSize {
	pub fn parse<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		let size = input.parse_comma_separated(|input| BgSize::parse(context, input))?;
		Ok(BackgroundSize { size })
	}
}

impl ToCss for BackgroundSize {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		let values: Vec<String> = self.size.iter().map(|v| v.to_css_string()).collect();
		dest.write_str(&values.join(", "))
	}
}

pub fn parse_declared<'i, 't>(
	context: &ParserContext,
	input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
	BackgroundSize::parse(context, input).map(PropertyDeclaration::BackgroundSize)
}
