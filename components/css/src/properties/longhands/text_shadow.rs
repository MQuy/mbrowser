use cssparser::{Parser, ToCss};

use crate::parser::{parse_in_any_order, parse_item_if_missing, ParseError};
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::specified::color::Color;
use crate::values::specified::length::{Length, NonNegativeLength};

#[derive(Clone, Debug)]
pub struct SingleTextShadow {
	color: Color,
	shadow: (Length, Length, NonNegativeLength),
}

impl SingleTextShadow {
	pub fn parse<'i, 't>(context: &ParserContext, input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i>> {
		let mut color = None;
		let mut shadow = None;
		parse_in_any_order(
			input,
			&mut [
				&mut |input| parse_item_if_missing(input, &mut color, &mut |_, input| Color::parse(context, input)),
				&mut |input| {
					parse_item_if_missing(input, &mut shadow, &mut |_, input| {
						let horizontal = Length::parse(context, input)?;
						let vertical = Length::parse(context, input)?;
						let blur = NonNegativeLength::parse(context, input).map_or("0px".into(), |value| value);
						Ok((horizontal, vertical, blur))
					})
				},
			],
		);

		if let Some(shadow) = shadow {
			Ok(SingleTextShadow {
				color: color.map_or(Color::CurrentColor, |v| v),
				shadow,
			})
		} else {
			Err(input.new_custom_error(StyleParseErrorKind::UnspecifiedError))
		}
	}
}

impl ToCss for SingleTextShadow {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		self.color.to_css(dest)?;
		dest.write_char(' ')?;
		dest.write_fmt(format_args!(
			"{} {} {}",
			self.shadow.0.to_css_string(),
			self.shadow.1.to_css_string(),
			self.shadow.2.to_css_string()
		))
	}
}

/// https://drafts.csswg.org/css-text-decor/#text-shadow-property
#[derive(Clone, Debug)]
pub struct TextShadow(Vec<SingleTextShadow>);

impl TextShadow {
	pub fn parse<'i, 't>(context: &ParserContext, input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i>> {
		input
			.try_parse(|input| {
				input.expect_ident_matching("none")?;
				Ok(TextShadow(vec![]))
			})
			.or_else(|_err: ParseError<'i>| {
				let values = input.parse_comma_separated(|input| SingleTextShadow::parse(context, input))?;
				Ok(TextShadow(values))
			})
	}
}

impl ToCss for TextShadow {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		match self.0.len() {
			0 => dest.write_str("none"),
			_ => {
				let values: Vec<String> = self.0.iter().map(|v| v.to_css_string()).collect();
				dest.write_str(&values.join(", "))
			},
		}
	}
}

pub fn parse_declared<'i, 't>(
	context: &ParserContext,
	input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
	TextShadow::parse(context, input).map(PropertyDeclaration::TextShadow)
}
