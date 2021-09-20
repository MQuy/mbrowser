use cssparser::{Parser, ToCss};

use crate::parser::{parse_repeated, ParseError};
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::color::Color;
use crate::values::length::{Length, NonNegativeLength};
use crate::values::number::{NonNegativeNumberOrPercentage, Zero};
use crate::values::specified::angle::Angle;
use crate::values::url::CssUrl;

#[derive(Clone, Debug)]
pub struct DropShadow {
	color: Color,
	lengths: (Length, Length, Length),
}

impl DropShadow {
	pub fn parse<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		let color = input
			.try_parse(|input| Color::parse(context, input))
			.map_or(Color::Transparent, |color| color);
		let horizontal = Length::parse(context, input)?;
		let vertical = Length::parse(context, input)?;
		let blur = input
			.try_parse(|input| Length::parse(context, input))
			.map_or("0px".into(), |length| length);
		Ok(DropShadow {
			color,
			lengths: (horizontal, vertical, blur),
		})
	}
}

impl ToCss for DropShadow {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		self.color.to_css(dest)?;
		dest.write_char(' ')?;
		self.lengths.0.to_css(dest)?;
		dest.write_char(' ')?;
		self.lengths.1.to_css(dest)?;
		dest.write_char(' ')?;
		self.lengths.2.to_css(dest)
	}
}

#[derive(Clone, Debug)]
pub enum FilterFunction {
	Blur(NonNegativeLength),
	Brightness(NonNegativeNumberOrPercentage),
	Contrast(NonNegativeNumberOrPercentage),
	DropShadow(DropShadow),
	Grayscale(NonNegativeNumberOrPercentage),
	HueRotate(Angle),
	Invert(NonNegativeNumberOrPercentage),
	Opacity(NonNegativeNumberOrPercentage),
	Saturate(NonNegativeNumberOrPercentage),
	Sepia(NonNegativeNumberOrPercentage),
}

impl FilterFunction {
	pub fn parse<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		input
			.try_parse(|input| {
				let length = FilterFunction::parse_argugment(
					input,
					|input| NonNegativeLength::parse(context, input),
					"blur",
					"0px".into(),
				)?;
				Ok(FilterFunction::Blur(length))
			})
			.or_else(|_err: ParseError<'i>| {
				let value = input.try_parse(|input| {
					FilterFunction::parse_argugment(
						input,
						|input| NonNegativeNumberOrPercentage::parse(context, input),
						"brightness",
						"1".into(),
					)
				})?;
				Ok(FilterFunction::Brightness(value))
			})
			.or_else(|_err: ParseError<'i>| {
				let value = input.try_parse(|input| {
					FilterFunction::parse_argugment(
						input,
						|input| NonNegativeNumberOrPercentage::parse(context, input),
						"contrast",
						"1".into(),
					)
				})?;
				Ok(FilterFunction::Contrast(value))
			})
			.or_else(|_err: ParseError<'i>| {
				let value = input.try_parse(|input| {
					input.expect_function_matching("drop-shadow")?;
					input.parse_nested_block(|input| DropShadow::parse(context, input))
				})?;
				Ok(FilterFunction::DropShadow(value))
			})
			.or_else(|_err: ParseError<'i>| {
				let value = input.try_parse(|input| {
					FilterFunction::parse_argugment(
						input,
						|input| NonNegativeNumberOrPercentage::parse(context, input),
						"grayscale",
						"1".into(),
					)
				})?;
				Ok(FilterFunction::Grayscale(value))
			})
			.or_else(|_err: ParseError<'i>| {
				let value = input.try_parse(|input| {
					FilterFunction::parse_argugment(
						input,
						|input| {
							input
								.try_parse(|input| Angle::parse(context, input))
								.or_else(|_err| {
									Zero::parse(context, input)?;
									Ok(Angle::Deg(0.0))
								})
						},
						"hue-rotate",
						"0deg".into(),
					)
				})?;
				Ok(FilterFunction::HueRotate(value))
			})
			.or_else(|_err: ParseError<'i>| {
				let value = input.try_parse(|input| {
					FilterFunction::parse_argugment(
						input,
						|input| NonNegativeNumberOrPercentage::parse(context, input),
						"invert",
						"1".into(),
					)
				})?;
				Ok(FilterFunction::Invert(value))
			})
			.or_else(|_err: ParseError<'i>| {
				let value = input.try_parse(|input| {
					FilterFunction::parse_argugment(
						input,
						|input| NonNegativeNumberOrPercentage::parse(context, input),
						"opacity",
						"1".into(),
					)
				})?;
				Ok(FilterFunction::Opacity(value))
			})
			.or_else(|_err: ParseError<'i>| {
				let value = input.try_parse(|input| {
					FilterFunction::parse_argugment(
						input,
						|input| NonNegativeNumberOrPercentage::parse(context, input),
						"saturate",
						"1".into(),
					)
				})?;
				Ok(FilterFunction::Saturate(value))
			})
			.or_else(|_err: ParseError<'i>| {
				let value = input.try_parse(|input| {
					FilterFunction::parse_argugment(
						input,
						|input| NonNegativeNumberOrPercentage::parse(context, input),
						"sepia",
						"1".into(),
					)
				})?;
				Ok(FilterFunction::Sepia(value))
			})
	}

	fn parse_argugment<'i, 't, F, T>(
		input: &mut Parser<'i, 't>,
		arg_parser: F,
		name: &str,
		default: T,
	) -> Result<T, ParseError<'i>>
	where
		F: for<'a, 'b> Fn(&mut Parser<'a, 'b>) -> Result<T, ParseError<'a>>,
	{
		input.expect_function_matching(name)?;
		input.parse_nested_block(|input| {
			let value = input
				.try_parse(|input| arg_parser(input))
				.map_or(default, |length| length);
			Ok(value)
		})
	}
}

impl ToCss for FilterFunction {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		match self {
			FilterFunction::Blur(value) => {
				dest.write_fmt(format_args!("blur({})", value.to_css_string()))
			},
			FilterFunction::Brightness(value) => {
				dest.write_fmt(format_args!("brightness({})", value.to_css_string()))
			},
			FilterFunction::Contrast(value) => {
				dest.write_fmt(format_args!("contrast({})", value.to_css_string()))
			},
			FilterFunction::DropShadow(value) => {
				dest.write_fmt(format_args!("drop-shadow({})", value.to_css_string()))
			},
			FilterFunction::Grayscale(value) => {
				dest.write_fmt(format_args!("grayscale({})", value.to_css_string()))
			},
			FilterFunction::HueRotate(value) => {
				dest.write_fmt(format_args!("hue-rotate({})", value.to_css_string()))
			},
			FilterFunction::Invert(value) => {
				dest.write_fmt(format_args!("invert({})", value.to_css_string()))
			},
			FilterFunction::Opacity(value) => {
				dest.write_fmt(format_args!("opacity({})", value.to_css_string()))
			},
			FilterFunction::Saturate(value) => {
				dest.write_fmt(format_args!("saturate({})", value.to_css_string()))
			},
			FilterFunction::Sepia(value) => {
				dest.write_fmt(format_args!("sepia({})", value.to_css_string()))
			},
		}
	}
}

#[derive(Clone, Debug)]
pub enum FilterFunctionOrUrl {
	Function(FilterFunction),
	Url(CssUrl),
}

impl FilterFunctionOrUrl {
	pub fn parse<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		input
			.try_parse(|input| {
				let url = CssUrl::parse(context, input)?;
				Ok(FilterFunctionOrUrl::Url(url))
			})
			.or_else(|_err: ParseError<'i>| {
				let function = FilterFunction::parse(context, input)?;
				Ok(FilterFunctionOrUrl::Function(function))
			})
	}
}

impl ToCss for FilterFunctionOrUrl {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		match self {
			FilterFunctionOrUrl::Function(value) => value.to_css(dest),
			FilterFunctionOrUrl::Url(value) => value.to_css(dest),
		}
	}
}

/// https://drafts.fxtf.org/filter-effects/#FilterProperty
#[derive(Clone, Debug)]
pub enum Filter {
	None,
	List(Vec<FilterFunctionOrUrl>),
}

impl Filter {
	pub fn parse<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		input
			.try_parse(|input| {
				input.expect_ident_matching("none")?;
				Ok(Filter::None)
			})
			.or_else(|_err: ParseError<'i>| {
				let filters = parse_repeated(
					input,
					&mut |input| FilterFunctionOrUrl::parse(context, input),
					1,
				)?;
				Ok(Filter::List(filters))
			})
	}
}

impl ToCss for Filter {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		match self {
			Filter::None => dest.write_str("none"),
			Filter::List(list) => {
				for (index, value) in list.iter().enumerate() {
					if index > 0 {
						dest.write_char(' ')?;
					}
					value.to_css(dest)?;
				}
				Ok(())
			},
		}
	}
}

pub fn parse_declared<'i, 't>(
	context: &ParserContext,
	input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
	Filter::parse(context, input).map(PropertyDeclaration::Filter)
}
