use cssparser::{match_ignore_ascii_case, Parser, ToCss, Token, _cssparser_internal_to_lowercase};

use crate::parser::ParseError;
use crate::properties::declaration::property_keywords_impl;
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::number::{NonNegativeNumber, Number};

#[derive(Clone)]
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

#[derive(Clone, PartialEq)]
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

#[derive(Clone)]
#[repr(u8, C)]
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
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        input
            .try_parse(|input| {
                let keyword = EasingKeyword::parse(input)?;
                Ok(EasingFunction::Keyword(keyword))
            })
            .or_else(|_err: ParseError<'i>| {
                let x1 = Number::parse_in_range(context, input, 0.0.into(), 1.0.into())?;
                let y1 = Number::parse(context, input)?;
                let x2 = Number::parse_in_range(context, input, 0.0.into(), 1.0.into())?;
                let y2 = Number::parse(context, input)?;
                Ok(EasingFunction::CubicBezier { x1, y1, x2, y2 })
            })
            .or_else(|_err: ParseError<'i>| {
                let intervals = NonNegativeNumber::parse(context, input)?;
                let position = StepPosition::parse(input)?;
                let lower_limit = if position == StepPosition::JumpNone {
                    1
                } else {
                    0
                };
                if intervals > lower_limit {
                    Ok(EasingFunction::Steps(intervals, position))
                } else {
                    Err(input.new_custom_error(StyleParseErrorKind::UnspecifiedError))
                }
            })
    }
}
