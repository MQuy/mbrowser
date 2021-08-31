use cssparser::{Parser, ToCss, Token, _cssparser_internal_to_lowercase, match_ignore_ascii_case};

use crate::parser::{parse_in_any_order, parse_item_if_missing, ParseError};
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::number::Number;
use crate::values::specified::angle::Angle;

#[derive(Clone)]
pub enum NumberOrKeyword {
	Number(Number),
	X,
	Y,
	Z,
}

impl NumberOrKeyword {
	pub fn parse<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		let location = input.current_source_location();
		let token = input.next()?.clone();
		let coordinate = match &token {
			Token::Ident(ident) => match_ignore_ascii_case! { ident,
				"x" => NumberOrKeyword::X,
				"y" => NumberOrKeyword::Y,
				"z" => NumberOrKeyword::Z,
				_ => return Err(location.new_custom_error(
					StyleParseErrorKind::UnexpectedToken(token.clone()),
				))
			},
			Token::Number { value: x, .. } => {
				let value = Number::parse(context, input)?;
				NumberOrKeyword::Number(value)
			},
			_ => {
				return Err(
					location.new_custom_error(StyleParseErrorKind::UnexpectedToken(token.clone()))
				)
			},
		};
		Ok(coordinate)
	}
}

impl ToCss for NumberOrKeyword {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		match self {
			NumberOrKeyword::Number(value) => dest.write_str(&value.to_string()),
			NumberOrKeyword::X => dest.write_char('x'),
			NumberOrKeyword::Y => dest.write_char('y'),
			NumberOrKeyword::Z => dest.write_char('z'),
		}
	}
}

/// https://drafts.csswg.org/css-transforms-2/#propdef-rotate
#[derive(Clone)]
pub enum Rotate {
	None,
	Rotate(Angle),
	Rotate3D(NumberOrKeyword, NumberOrKeyword, NumberOrKeyword, Angle),
}

impl Rotate {
	pub fn parse<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Rotate, ParseError<'i>> {
		input
			.try_parse(|input| {
				input.expect_ident_matching("none")?;
				Ok(Rotate::None)
			})
			.or_else(|_err: ParseError<'i>| {
				input.try_parse(|input| {
					let angle = Angle::parse(context, input)?;
					Ok(Rotate::Rotate(angle))
				})
			})
			.or_else(|_err: ParseError<'i>| {
				let mut angle = None;
				let mut coordinate = None;
				parse_in_any_order(
					input,
					&mut [
						&mut |input| {
							parse_item_if_missing(input, &mut angle, &mut |_, input| {
								Angle::parse(context, input)
							})
						},
						&mut |input| {
							parse_item_if_missing(input, &mut coordinate, &mut |_, input| {
								let x = NumberOrKeyword::parse(context, input)?;
								let y = NumberOrKeyword::parse(context, input)?;
								let z = NumberOrKeyword::parse(context, input)?;
								Ok((x, y, z))
							})
						},
					],
				);
				if let (Some((x, y, z)), Some(angle)) = (coordinate, angle) {
					Ok(Rotate::Rotate3D(x, y, z, angle))
				} else {
					Err(input.new_custom_error(StyleParseErrorKind::UnspecifiedError))
				}
			})
	}
}

impl ToCss for Rotate {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		match self {
			Rotate::None => dest.write_str("none"),
			Rotate::Rotate(value) => value.to_css(dest),
			Rotate::Rotate3D(x, y, z, angle) => {
				x.to_css(dest)?;
				dest.write_char(' ')?;
				y.to_css(dest)?;
				dest.write_char(' ')?;
				z.to_css(dest)?;
				dest.write_char(' ')?;
				angle.to_css(dest)
			},
		}
	}
}

pub fn parse_declared<'i, 't>(
	context: &ParserContext,
	input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
	Rotate::parse(context, input).map(PropertyDeclaration::Rotate)
}
