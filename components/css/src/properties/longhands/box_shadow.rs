use cssparser::{Parser, ToCss};

use crate::parser::{parse_in_any_order, parse_item_if_missing, ParseError};
use crate::properties::declaration::PropertyDeclaration;
use crate::str::convert_options_to_string;
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::specified::color::Color;
use crate::values::specified::length::{Length, NonNegativeLength};

#[derive(Clone, Debug)]
pub struct Shadow {
	inset: bool,
	length: (Length, Length, NonNegativeLength, Length),
	color: Option<Color>,
}

impl Shadow {
	pub fn parse<'i, 't, 'a>(context: &'a ParserContext, input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i>> {
		let mut color = None;
		let mut inset = None;
		let mut length = None;

		parse_in_any_order(
			input,
			&mut [
				&mut |input| {
					parse_item_if_missing(input, &mut inset, &mut |_, input| {
						input
							.expect_ident_matching("inset")
							.map_err(|_err| input.new_custom_error(StyleParseErrorKind::UnspecifiedError))
					})
				},
				&mut |input| {
					parse_item_if_missing(input, &mut length, &mut |_, input| {
						let horizontal = Length::parse(context, input)?;
						let vertical = Length::parse(context, input)?;
						let blur = input
							.try_parse(|input| NonNegativeLength::parse(context, input))
							.map_or("0px".into(), |length| length);
						let spread = input
							.try_parse(|input| Length::parse(context, input))
							.map_or("0px".into(), |length| length);
						Ok((horizontal, vertical, blur, spread))
					})
				},
				&mut |input| parse_item_if_missing(input, &mut color, &mut |_, input| Color::parse(context, input)),
			],
		);

		if let Some(length) = length {
			Ok(Shadow {
				inset: inset.is_some(),
				color,
				length,
			})
		} else {
			Err(input.new_custom_error(StyleParseErrorKind::UnspecifiedError))
		}
	}
}

impl ToCss for Shadow {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		let color = self.color.as_ref().map(|v| v.to_css_string());
		let length = Some(std::format!(
			"{} {} {} {}",
			self.length.0.to_css_string(),
			self.length.1.to_css_string(),
			self.length.2.to_css_string(),
			self.length.3.to_css_string(),
		));
		let inset = if self.inset { Some("inset".to_string()) } else { None };
		dest.write_str(&convert_options_to_string(vec![color, length, inset], " "))
	}
}

/// https://drafts.csswg.org/css-backgrounds/#box-shadow
#[derive(Clone, Debug)]
pub enum BoxShadow {
	None,
	Shadow(Vec<Shadow>),
}

impl BoxShadow {
	pub fn parse<'i, 't>(context: &ParserContext, input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i>> {
		input
			.try_parse(|input| {
				input.expect_ident_matching("none")?;
				Ok(BoxShadow::None)
			})
			.or_else(|_err: ParseError<'i>| {
				let shadows = input.parse_comma_separated(|input| {
					let value = Shadow::parse(context, input)?;
					Ok(value)
				})?;
				Ok(BoxShadow::Shadow(shadows))
			})
	}
}

impl ToCss for BoxShadow {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		match self {
			BoxShadow::None => dest.write_str("none"),
			BoxShadow::Shadow(value) => dest.write_str(
				&value
					.iter()
					.map(|v| v.to_css_string())
					.collect::<Vec<String>>()
					.join(", "),
			),
		}
	}
}

pub fn parse_declared<'i, 't>(
	context: &ParserContext,
	input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
	BoxShadow::parse(context, input).map(PropertyDeclaration::BoxShadow)
}
