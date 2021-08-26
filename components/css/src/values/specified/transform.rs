use cssparser::{Parser, ToCss, _cssparser_internal_to_lowercase, match_ignore_ascii_case};

use super::angle::Angle;
use crate::parser::ParseError;
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::length::LengthPercentage;
use crate::values::number::{Number, Zero};

#[derive(Clone)]
pub enum AngleOrZero {
	Angle(Angle),
	Zero,
}

impl AngleOrZero {
	pub fn parse<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		input
			.try_parse(|input| {
				let angle = Angle::parse(context, input)?;
				Ok(AngleOrZero::Angle(angle))
			})
			.or_else(|_err: ParseError<'i>| {
				Zero::parse(context, input)?;
				Ok(AngleOrZero::Zero)
			})
	}
}

impl ToCss for AngleOrZero {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		match self {
			AngleOrZero::Angle(value) => value.to_css(dest),
			AngleOrZero::Zero => dest.write_char('0'),
		}
	}
}

/// https://drafts.csswg.org/css-transforms-1/#two-d-transform-functions
#[derive(Clone)]
pub enum TransformFunction {
	Matrix(Number, Number, Number, Number, Number, Number),
	Translate(LengthPercentage, LengthPercentage),
	TranslateX(LengthPercentage),
	TranslateY(LengthPercentage),
	Scale(Number, Number),
	ScaleX(Number),
	ScaleY(Number),
	Rotate(AngleOrZero),
	Skew(AngleOrZero, AngleOrZero),
	SkewX(AngleOrZero),
	SkewY(AngleOrZero),
}

impl TransformFunction {
	pub fn parse<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		let name = input.expect_function()?.clone();
		input.parse_nested_block(|input| {
			match_ignore_ascii_case! { &name,
				"matrix" => TransformFunction::parse_matrix(context, input),
				"translate" => TransformFunction::parse_translate(context, input),
				"translatex" => TransformFunction::parse_translate_x(context, input),
				"translatey" => TransformFunction::parse_translate_y(context, input),
				"scale" => TransformFunction::parse_scale(context, input),
				"scalex" => TransformFunction::parse_scale_x(context, input),
				"scaley" => TransformFunction::parse_scale_y(context, input),
				"rotate" => TransformFunction::parse_rotate(context, input),
				"skew" => TransformFunction::parse_skew(context, input),
				"skewx" => TransformFunction::parse_skew_x(context, input),
				"skewy" => TransformFunction::parse_skew_y(context, input),
				_ => return Err(input.new_custom_error(StyleParseErrorKind::UnexpectedFunction(name.clone())))
			}
		})
	}

	fn parse_matrix<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		let a = Number::parse(context, input)?;
		let b = Number::parse(context, input)?;
		let c = Number::parse(context, input)?;
		let d = Number::parse(context, input)?;
		let e = Number::parse(context, input)?;
		let f = Number::parse(context, input)?;
		Ok(TransformFunction::Matrix(a, b, c, d, e, f))
	}

	fn parse_translate<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		let tx = LengthPercentage::parse(context, input)?;
		let ty = LengthPercentage::parse(context, input)
			.map_or(LengthPercentage::Length("0".into()), |v| v);
		Ok(TransformFunction::Translate(tx, ty))
	}

	fn parse_translate_x<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		let x = LengthPercentage::parse(context, input)?;
		Ok(TransformFunction::TranslateX(x))
	}

	fn parse_translate_y<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		let y = LengthPercentage::parse(context, input)?;
		Ok(TransformFunction::TranslateY(y))
	}

	fn parse_scale<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		let sx = Number::parse(context, input)?;
		let sy = Number::parse(context, input).map_or(sx.clone(), |v| v);
		Ok(TransformFunction::Scale(sx, sy))
	}

	fn parse_scale_x<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		let sx = Number::parse(context, input)?;
		Ok(TransformFunction::ScaleX(sx))
	}

	fn parse_scale_y<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		let sy = Number::parse(context, input)?;
		Ok(TransformFunction::ScaleY(sy))
	}

	fn parse_rotate<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		let angle = AngleOrZero::parse(context, input)?;
		Ok(TransformFunction::Rotate(angle))
	}

	fn parse_skew<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		let ax = AngleOrZero::parse(context, input)?;
		let ay = AngleOrZero::parse(context, input).map_or(AngleOrZero::Zero, |v| v);
		Ok(TransformFunction::Skew(ax, ay))
	}

	fn parse_skew_x<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		let ax = AngleOrZero::parse(context, input)?;
		Ok(TransformFunction::SkewX(ax))
	}

	fn parse_skew_y<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		let ay = AngleOrZero::parse(context, input)?;
		Ok(TransformFunction::SkewY(ay))
	}
}

impl ToCss for TransformFunction {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		match self {
			TransformFunction::Matrix(a, b, c, d, e, f) => {
				dest.write_fmt(format_args!("matrix({} {} {} {} {} {}", a, b, c, d, e, f))
			},
			TransformFunction::Translate(tx, ty) => dest.write_fmt(format_args!(
				"translate({}, {})",
				tx.to_css_string(),
				ty.to_css_string()
			)),
			TransformFunction::TranslateX(x) => {
				dest.write_fmt(format_args!("translateX({})", x.to_css_string()))
			},
			TransformFunction::TranslateY(y) => {
				dest.write_fmt(format_args!("translateY({})", y.to_css_string()))
			},
			TransformFunction::Scale(sx, sy) => {
				dest.write_fmt(format_args!("scale({}, {})", sx, sy))
			},
			TransformFunction::ScaleX(sx) => dest.write_fmt(format_args!("scaleX({})", sx)),
			TransformFunction::ScaleY(sy) => dest.write_fmt(format_args!("scaleY({})", sy)),
			TransformFunction::Rotate(angle) => {
				dest.write_fmt(format_args!("rotate({})", angle.to_css_string()))
			},
			TransformFunction::Skew(ax, ay) => dest.write_fmt(format_args!(
				"skew({}, {})",
				ax.to_css_string(),
				ay.to_css_string()
			)),
			TransformFunction::SkewX(ax) => {
				dest.write_fmt(format_args!("skewX({})", ax.to_css_string()))
			},
			TransformFunction::SkewY(ay) => {
				dest.write_fmt(format_args!("skewY({})", ay.to_css_string()))
			},
		}
	}
}
