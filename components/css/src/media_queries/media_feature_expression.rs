use core::fmt;
use std::fmt::Write;

use cssparser::{CowRcStr, Parser, ParserState, Token};

use super::media_condition::MediaCondition;
use super::media_features::{
	DisplayMode, Enumerated, ForcedColors, Hover, MediaFeatureDescription, OverflowBlock,
	OverflowInline, Pointer, PrefersColorScheme, PrefersContrast, PrefersReducedMotion, Scan,
};
use crate::css_writer::{CssWriter, ToCss};
use crate::media_queries::media_features::{Evaluator, Orientation, MEDIA_FEATURES};
use crate::parser::ParseError;
use crate::str::starts_with_ignore_ascii_case;
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::layout::Resolution;
use crate::values::length::Length;
use crate::values::number::{Integer, Number};
use crate::values::percentage::Ratio;
use crate::values::{CSSFloat, Ident};

/// The kind of matching that should be performed on a media feature value.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Range {
	/// At least the specified value.
	Min,
	/// At most the specified value.
	Max,
}

/// The operator that was specified in this media feature.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Operator {
	/// =
	Equal,
	/// >
	GreaterThan,
	/// >=
	GreaterThanEqual,
	/// <
	LessThan,
	/// <=
	LessThanEqual,
}

impl Operator {
	pub fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i>> {
		let token = input.next()?;
		match token {
			Token::Delim('=') => Ok(Operator::Equal),
			Token::Delim('>') => input
				.try_parse(|input| {
					input.expect_delim('=')?;
					Ok(Operator::GreaterThanEqual)
				})
				.or_else(|_err: ParseError<'i>| Ok(Operator::GreaterThan)),
			Token::Delim('<') => input
				.try_parse(|input| {
					input.expect_delim('=')?;
					Ok(Operator::LessThanEqual)
				})
				.or_else(|_err: ParseError<'i>| Ok(Operator::LessThan)),

			_ => Err(input.new_custom_error(StyleParseErrorKind::MediaQueryExpectedToken)),
		}
	}
}

/// Either a `Range` or an `Operator`.
///
/// Ranged media features are not allowed with operations (that'd make no
/// sense).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RangeOrOperator {
	/// A `Range`.
	Range(Range),
	/// An `Operator`.
	Operator(Operator),
}

impl RangeOrOperator {
	pub fn negative(&self) -> Self {
		match self {
			RangeOrOperator::Range(range) => RangeOrOperator::Range(match range {
				Range::Min => Range::Max,
				Range::Max => Range::Min,
			}),
			RangeOrOperator::Operator(operator) => RangeOrOperator::Operator(match operator {
				Operator::Equal => Operator::Equal,
				Operator::GreaterThan => Operator::LessThan,
				Operator::GreaterThanEqual => Operator::LessThanEqual,
				Operator::LessThan => Operator::GreaterThan,
				Operator::LessThanEqual => Operator::GreaterThanEqual,
			}),
		}
	}
}

/// A feature expression contains a reference to the media feature, the value
/// the media query contained, and the range to evaluate.
#[derive(Clone, Debug, PartialEq)]
pub struct MediaFeatureExpression {
	feature_index: usize,
	value: Option<MediaExpressionValue>,
	range_or_operator: Option<RangeOrOperator>,
}

impl MediaFeatureExpression {
	pub fn feature(&self) -> &'static MediaFeatureDescription {
		&MEDIA_FEATURES[self.feature_index]
	}

	/// https://drafts.csswg.org/mediaqueries-5/#ref-for-typedef-media-feature
	pub fn parse<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<MediaCondition, ParseError<'i>> {
		input
			.try_parse(|input| {
				Ok(MediaCondition::Feature(
					MediaFeatureExpression::parse_plain(context, input)?,
				))
			})
			.or_else(|_err: ParseError<'i>| {
				input
					.try_parse(|input| MediaFeatureExpression::parse_range(context, input))
					.or_else(|_err: ParseError<'i>| {
						Ok(MediaCondition::Feature(
							MediaFeatureExpression::parse_boolean(input)?,
						))
					})
			})
	}

	/// https://drafts.csswg.org/mediaqueries-5/#ref-for-typedef-mf-plain
	fn parse_plain<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		let (feature_index, feature, range) = MediaFeatureExpression::parse_feature_name(input)?;

		input.expect_colon()?;

		let value = MediaExpressionValue::parse(context, input, feature)?;

		let range_or_operator = if let Some(range) = range {
			RangeOrOperator::Range(range)
		} else {
			RangeOrOperator::Operator(Operator::Equal)
		};
		Ok(MediaFeatureExpression {
			feature_index,
			value: Some(value),
			range_or_operator: Some(range_or_operator),
		})
	}

	/// https://drafts.csswg.org/mediaqueries-5/#ref-for-typedef-mf-boolean
	fn parse_boolean<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i>> {
		let (feature_index, _feature, range) = MediaFeatureExpression::parse_feature_name(input)?;
		assert!(range.is_none());

		Ok(MediaFeatureExpression {
			feature_index,
			value: None,
			range_or_operator: None,
		})
	}

	/// https://drafts.csswg.org/mediaqueries-5/#ref-for-typedef-mf-range
	fn parse_range<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<MediaCondition, ParseError<'i>> {
		input
			.try_parse(|input| {
				let (feature_index, feature, range) =
					MediaFeatureExpression::parse_feature_name(input)?;
				assert!(range.is_none());
				let range_operator = MediaFeatureExpression::parse_comparison(input, range)?;
				let value = MediaExpressionValue::parse(context, input, feature)?;
				Ok(MediaCondition::Feature(MediaFeatureExpression {
					value: Some(value),
					feature_index,
					range_or_operator: Some(range_operator),
				}))
			})
			.or_else(|_err: ParseError<'i>| {
				let featurable = MediaFeatureExpression::peek_into_feature_name(input);

				if featurable.is_none() {
					return Err(input.new_custom_error(
						StyleParseErrorKind::MediaQueryExpectedFeatureName(CowRcStr::from("")),
					));
				}
				let (feature_range, next_state) = featurable.unwrap();
				let left_value = MediaExpressionValue::parse(context, input, feature_range.1)?;
				// 600px < height
				// -> value = 600px, op = <, name = height
				// we have to reverse op to keep the expression correct
				// -> value = 600px, op = >, name = height
				let left_operator =
					MediaFeatureExpression::parse_comparison(input, feature_range.2)?;
				let left_media_condition = MediaCondition::Feature(MediaFeatureExpression {
					value: Some(left_value),
					feature_index: feature_range.0,
					range_or_operator: Some(left_operator.negative()),
				});
				input.reset(&next_state);

				input
					.try_parse(|input| {
						let right_operator =
							MediaFeatureExpression::parse_comparison(input, feature_range.2)?;
						let right_value =
							MediaExpressionValue::parse(context, input, feature_range.1)?;
						let right_media_condition =
							MediaCondition::Feature(MediaFeatureExpression {
								value: Some(right_value),
								feature_index: feature_range.0,
								range_or_operator: Some(right_operator),
							});
						Ok(MediaCondition::Operation(
							vec![left_media_condition.clone(), right_media_condition],
							super::media_condition::Operator::And,
						))
					})
					.or_else(|_err: ParseError<'i>| Ok(left_media_condition))
			})
	}

	fn parse_comparison<'i, 't>(
		input: &mut Parser<'i, 't>,
		range: Option<Range>,
	) -> Result<RangeOrOperator, ParseError<'i>> {
		let operator = Operator::parse(input)?;
		match range {
			Some(range) => match operator {
				Operator::Equal => Ok(if range == Range::Max {
					RangeOrOperator::Operator(Operator::GreaterThanEqual)
				} else {
					RangeOrOperator::Operator(Operator::LessThanEqual)
				}),
				_ => {
					return Err(
						input.new_custom_error(StyleParseErrorKind::MediaQueryUnexpectedOperator)
					)
				},
			},
			None => Ok(RangeOrOperator::Operator(operator)),
		}
	}

	fn parse_feature_name<'i, 't>(
		input: &mut Parser<'i, 't>,
	) -> Result<(usize, &'static MediaFeatureDescription, Option<Range>), ParseError<'i>> {
		let ident = input.expect_ident()?.clone();
		let (range, feature_name) = if starts_with_ignore_ascii_case(ident.as_ref(), "min-") {
			(Some(Range::Min), &ident[4..])
		} else if starts_with_ignore_ascii_case(ident.as_ref(), "max-") {
			(Some(Range::Max), &ident[4..])
		} else {
			(None, ident.as_ref())
		};

		let (feature_index, feature) = MEDIA_FEATURES
			.iter()
			.enumerate()
			.find(|(_index, feature)| feature.name == feature_name)
			.ok_or_else(|| {
				input.new_custom_error(StyleParseErrorKind::MediaQueryExpectedFeatureName(ident))
			})?;
		Ok((feature_index, feature, range))
	}

	fn peek_into_feature_name<'i, 't>(
		input: &mut Parser<'i, 't>,
	) -> Option<(
		(usize, &'static MediaFeatureDescription, Option<Range>),
		ParserState,
	)> {
		fn peekable<'i, 't>(
			input: &mut Parser<'i, 't>,
		) -> Option<(usize, &'static MediaFeatureDescription, Option<Range>)> {
			loop {
				if Operator::parse(input).is_ok() {
					if let Ok(feature_range) = MediaFeatureExpression::parse_feature_name(input) {
						assert!(feature_range.2.is_none());
						return Some(feature_range);
					}
					break;
				}
				if input.expect_exhausted().is_ok() {
					break;
				}
			}
			None
		}

		let prev_state = input.state();
		let feature_range = peekable(input);
		let cur_state = input.state();
		input.reset(&prev_state);

		feature_range.map(|feature_range| (feature_range, cur_state))
	}
}

impl ToCss for MediaFeatureExpression {
	fn to_css<W>(&self, dest: &mut CssWriter<W>) -> fmt::Result
	where
		W: std::fmt::Write,
	{
		match self.range_or_operator {
			Some(RangeOrOperator::Range(range)) => {
				dest.write_str(if range == Range::Max { "max-" } else { "min-" })?;
			},
			_ => (),
		}
		dest.write_str(self.feature().name)?;
		match self.range_or_operator {
			Some(RangeOrOperator::Operator(operator)) => match operator {
				Operator::Equal => dest.write_str(" = ")?,
				Operator::GreaterThan => dest.write_str(" > ")?,
				Operator::GreaterThanEqual => dest.write_str(" >= ")?,
				Operator::LessThan => dest.write_str(" < ")?,
				Operator::LessThanEqual => dest.write_str(" <= ")?,
			},
			Some(RangeOrOperator::Range(_)) => dest.write_str(": ")?,
			_ => (),
		};
		self.value.to_css(dest)
	}
}

#[derive(Clone, Debug, PartialEq)]
pub enum MediaExpressionValue {
	/// A length.
	Length(Length),
	/// A (non-negative) integer.
	Integer(u32),
	/// A boolean value, specified as an integer (i.e., either 0 or 1).
	BoolInteger(bool),
	/// A floating point value.
	Float(CSSFloat),
	/// A single non-negative number or two non-negative numbers separated by '/',
	/// with optional whitespace on either side of the '/'.
	NumberRatio(Ratio),
	/// A resolution.
	Resolution(Resolution),
	/// A keyword value.
	Enumerated(MediaExpressionValueEnumerated),
	/// An identifier.
	Ident(Ident),
}

impl MediaExpressionValue {
	fn parse<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
		description_value: &MediaFeatureDescription,
	) -> Result<Self, ParseError<'i>> {
		Ok(match &description_value.evaluator {
			Evaluator::Length => {
				let length = Length::parse_non_negative(context, input)?;
				MediaExpressionValue::Length(length)
			},
			Evaluator::Integer => {
				let integer = Integer::parse_non_negative(context, input)?;
				MediaExpressionValue::Integer(integer.value() as u32)
			},
			Evaluator::Float => {
				let number = Number::parse(context, input)?;
				MediaExpressionValue::Float(number.get())
			},
			Evaluator::BoolInteger => {
				let integer = Integer::parse_non_negative(context, input)?;
				let value = integer.value();
				if value > 1 {
					return Err(input.new_custom_error(StyleParseErrorKind::UnspecifiedError));
				}
				MediaExpressionValue::BoolInteger(value == 1)
			},
			Evaluator::NumberRatio => {
				let ratio = Ratio::parse(context, input)?;
				MediaExpressionValue::NumberRatio(ratio)
			},
			Evaluator::Resolution => {
				let resolution = Resolution::parse(context, input)?;
				MediaExpressionValue::Resolution(resolution)
			},
			Evaluator::Enumerated(enumerated) => MediaExpressionValue::Enumerated(
				MediaExpressionValueEnumerated::parse(input, enumerated)?,
			),
			Evaluator::Ident => {
				let ident = input.expect_ident()?;
				MediaExpressionValue::Ident(Ident::from(ident.as_ref()))
			},
		})
	}
}

impl ToCss for MediaExpressionValue {
	fn to_css<W>(&self, dest: &mut CssWriter<W>) -> fmt::Result
	where
		W: Write,
	{
		match self {
			MediaExpressionValue::Length(length) => {
				dest.write_str(&cssparser::ToCss::to_css_string(length))
			},
			MediaExpressionValue::Integer(value) => dest.write_str(&std::format!("{}", value)),
			MediaExpressionValue::BoolInteger(value) => dest.write_str(&std::format!("{}", value)),
			MediaExpressionValue::Float(value) => dest.write_str(&std::format!("{}", value)),
			MediaExpressionValue::NumberRatio(ratio) => {
				dest.write_str(&cssparser::ToCss::to_css_string(ratio))
			},
			MediaExpressionValue::Resolution(resolution) => {
				dest.write_str(&cssparser::ToCss::to_css_string(resolution))
			},
			MediaExpressionValue::Enumerated(enumerated) => enumerated.to_css(dest),
			MediaExpressionValue::Ident(ident) => {
				dest.write_str(&cssparser::ToCss::to_css_string(ident))
			},
		}
	}
}

#[derive(Clone, Debug, PartialEq)]
pub enum MediaExpressionValueEnumerated {
	Orientation(Orientation),
	DisplayMode(DisplayMode),
	Scan(Scan),
	PrefersReducedMotion(PrefersReducedMotion),
	PrefersContrast(PrefersContrast),
	ForcedColors(ForcedColors),
	OverflowBlock(OverflowBlock),
	OverflowInline(OverflowInline),
	PrefersColorScheme(PrefersColorScheme),
	Pointer(Pointer),
	AnyPointer(Pointer),
	Hover(Hover),
	AnyHover(Hover),
}

impl MediaExpressionValueEnumerated {
	fn parse<'i, 't>(
		input: &mut Parser<'i, 't>,
		enumerated: &Enumerated,
	) -> Result<Self, ParseError<'i>> {
		Ok(match enumerated {
			Enumerated::Orientation => {
				let orientation = Orientation::parse(input)?;
				MediaExpressionValueEnumerated::Orientation(orientation)
			},
			Enumerated::DisplayMode => {
				let display_mode = DisplayMode::parse(input)?;
				MediaExpressionValueEnumerated::DisplayMode(display_mode)
			},
			Enumerated::Scan => {
				let scan = Scan::parse(input)?;
				MediaExpressionValueEnumerated::Scan(scan)
			},
			Enumerated::PrefersReducedMotion => {
				let prefers_reduced_motion = PrefersReducedMotion::parse(input)?;
				MediaExpressionValueEnumerated::PrefersReducedMotion(prefers_reduced_motion)
			},
			Enumerated::PrefersContrast => {
				let prefers_contrats = PrefersContrast::parse(input)?;
				MediaExpressionValueEnumerated::PrefersContrast(prefers_contrats)
			},
			Enumerated::ForcedColors => {
				let forced_colors = ForcedColors::parse(input)?;
				MediaExpressionValueEnumerated::ForcedColors(forced_colors)
			},
			Enumerated::OverflowBlock => {
				let overflow_block = OverflowBlock::parse(input)?;
				MediaExpressionValueEnumerated::OverflowBlock(overflow_block)
			},
			Enumerated::OverflowInline => {
				let overflow_inline = OverflowInline::parse(input)?;
				MediaExpressionValueEnumerated::OverflowInline(overflow_inline)
			},
			Enumerated::PrefersColorScheme => {
				let prefers_color_scheme = PrefersColorScheme::parse(input)?;
				MediaExpressionValueEnumerated::PrefersColorScheme(prefers_color_scheme)
			},
			Enumerated::Pointer => {
				let pointer = Pointer::parse(input)?;
				MediaExpressionValueEnumerated::Pointer(pointer)
			},
			Enumerated::AnyPointer => {
				let pointer = Pointer::parse(input)?;
				MediaExpressionValueEnumerated::AnyPointer(pointer)
			},
			Enumerated::Hover => {
				let hover = Hover::parse(input)?;
				MediaExpressionValueEnumerated::Hover(hover)
			},
			Enumerated::AnyHover => {
				let hover = Hover::parse(input)?;
				MediaExpressionValueEnumerated::AnyHover(hover)
			},
		})
	}
}

impl ToCss for MediaExpressionValueEnumerated {
	fn to_css<W>(&self, dest: &mut CssWriter<W>) -> fmt::Result
	where
		W: Write,
	{
		match self {
			MediaExpressionValueEnumerated::Orientation(orientation) => orientation.to_css(dest),
			MediaExpressionValueEnumerated::DisplayMode(mode) => mode.to_css(dest),
			MediaExpressionValueEnumerated::Scan(scan) => scan.to_css(dest),
			MediaExpressionValueEnumerated::PrefersReducedMotion(motion) => motion.to_css(dest),
			MediaExpressionValueEnumerated::PrefersContrast(contrast) => contrast.to_css(dest),
			MediaExpressionValueEnumerated::ForcedColors(color) => color.to_css(dest),
			MediaExpressionValueEnumerated::OverflowBlock(block) => block.to_css(dest),
			MediaExpressionValueEnumerated::OverflowInline(inline) => inline.to_css(dest),
			MediaExpressionValueEnumerated::PrefersColorScheme(scheme) => scheme.to_css(dest),
			MediaExpressionValueEnumerated::Pointer(pointer) => pointer.to_css(dest),
			MediaExpressionValueEnumerated::AnyPointer(pointer) => pointer.to_css(dest),
			MediaExpressionValueEnumerated::Hover(hover) => hover.to_css(dest),
			MediaExpressionValueEnumerated::AnyHover(hover) => hover.to_css(dest),
		}
	}
}
