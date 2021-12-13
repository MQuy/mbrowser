use cssparser::{match_ignore_ascii_case, Parser, ToCss, Token, _cssparser_internal_to_lowercase};

use super::number::{NonNegativeNumber, Number};
use crate::parser::ParseError;
use crate::properties::declaration::property_keywords_impl;
use crate::stylesheets::rule_parser::StyleParseErrorKind;

#[derive(Clone, Debug)]
#[repr(u8)]
pub enum EasingKeyword {
	Linear,
	Ease,
	EaseIn,
	EaseOut,
	EaseInOut,
	StepStart,
	StepEnd,
}

property_keywords_impl! { EasingKeyword,
	EasingKeyword::Linear, "linear",
	EasingKeyword::Ease, "ease",
	EasingKeyword::EaseIn, "ease-in",
	EasingKeyword::EaseOut, "ease-out",
	EasingKeyword::EaseInOut, "ease-in-out",
	EasingKeyword::StepStart, "step-start",
	EasingKeyword::StepEnd, "step-end",
}

#[derive(Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum StepPosition {
	JumpStart,
	JumpEnd,
	JumpNone,
	JumpBoth,
	Start,
	End,
}

property_keywords_impl! { StepPosition,
	StepPosition::JumpStart, "jump-start",
	StepPosition::JumpEnd, "jump-end",
	StepPosition::JumpNone, "jump-none",
	StepPosition::JumpBoth, "jump-both",
	StepPosition::Start, "start",
	StepPosition::End, "end",
}

/// https://drafts.csswg.org/css-easing-1/#easing-functions
#[derive(Clone, Debug)]
pub enum EasingFunction {
	Keyword(EasingKeyword),
	CubicBezier {
		x1: Number,
		y1: Number,
		x2: Number,
		y2: Number,
	},
	Steps(NonNegativeNumber, StepPosition),
}

impl EasingFunction {
	pub fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i>> {
		input
			.try_parse(|input| {
				let keyword = EasingKeyword::parse(input)?;
				Ok(EasingFunction::Keyword(keyword))
			})
			.or_else(|_err: ParseError<'i>| {
				input.try_parse(|input| {
					input.expect_function_matching("cubic-bezier")?;
					input.parse_nested_block(|input| {
						let x1 = Number::parse_in_range(input, 0.0.into(), 1.0.into())?;
						input.expect_comma()?;
						let y1 = Number::parse(input)?;
						input.expect_comma()?;
						let x2 = Number::parse_in_range(input, 0.0.into(), 1.0.into())?;
						input.expect_comma()?;
						let y2 = Number::parse(input)?;
						Ok(EasingFunction::CubicBezier { x1, y1, x2, y2 })
					})
				})
			})
			.or_else(|_err: ParseError<'i>| {
				input.expect_function_matching("steps")?;
				input.parse_nested_block(|input| {
					let intervals = NonNegativeNumber::parse(input)?;
					let position = input
						.try_parse(|input| {
							input.expect_comma()?;
							StepPosition::parse(input)
						})
						.map_or(StepPosition::End, |v| v);
					let lower_limit = if position == StepPosition::JumpNone { 1.0 } else { 0.0 };
					if intervals > lower_limit {
						Ok(EasingFunction::Steps(intervals, position))
					} else {
						Err(input.new_custom_error(StyleParseErrorKind::UnspecifiedError))
					}
				})
			})
	}
}

impl ToCss for EasingFunction {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		match self {
			EasingFunction::Keyword(keyword) => keyword.to_css(dest),
			EasingFunction::CubicBezier { x1, y1, x2, y2 } => {
				dest.write_fmt(format_args!("cubic-bezier({}, {}, {}, {})", x1, y1, x2, y2))
			},
			EasingFunction::Steps(value, position) => dest.write_fmt(format_args!(
				"steps({}, {})",
				value.to_css_string(),
				position.to_css_string()
			)),
		}
	}
}
