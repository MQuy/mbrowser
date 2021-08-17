use common::url::BrowserUrl;
use cssparser::Parser;

use crate::parser::{parse_repeated, ParseError};
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::color::Color;
use crate::values::length::{Length, NonNegativeLength};
use crate::values::number::NonNegativeNumberOrPercentage;
use crate::values::specified::angle::Angle;

#[derive(Clone)]
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
            .map_or("0".into(), |length| length);
        Ok(DropShadow {
            color,
            lengths: (horizontal, vertical, blur),
        })
    }
}

#[derive(Clone)]
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
                let value = FilterFunction::parse_argugment(
                    input,
                    |input| NonNegativeNumberOrPercentage::parse(context, input),
                    "brightness",
                    "1".into(),
                )?;
                Ok(FilterFunction::Brightness(value))
            })
            .or_else(|_err: ParseError<'i>| {
                let value = FilterFunction::parse_argugment(
                    input,
                    |input| NonNegativeNumberOrPercentage::parse(context, input),
                    "contrast",
                    "1".into(),
                )?;
                Ok(FilterFunction::Contrast(value))
            })
            .or_else(|_err: ParseError<'i>| {
                let value = DropShadow::parse(context, input)?;
                Ok(FilterFunction::DropShadow(value))
            })
            .or_else(|_err: ParseError<'i>| {
                let value = FilterFunction::parse_argugment(
                    input,
                    |input| NonNegativeNumberOrPercentage::parse(context, input),
                    "grayscale",
                    "1".into(),
                )?;
                Ok(FilterFunction::Grayscale(value))
            })
            .or_else(|_err: ParseError<'i>| {
                let value = Angle::parse(context, input)?;
                Ok(FilterFunction::HueRotate(value))
            })
            .or_else(|_err: ParseError<'i>| {
                let value = FilterFunction::parse_argugment(
                    input,
                    |input| NonNegativeNumberOrPercentage::parse(context, input),
                    "invert",
                    "1".into(),
                )?;
                Ok(FilterFunction::Invert(value))
            })
            .or_else(|_err: ParseError<'i>| {
                let value = FilterFunction::parse_argugment(
                    input,
                    |input| NonNegativeNumberOrPercentage::parse(context, input),
                    "opacity",
                    "1".into(),
                )?;
                Ok(FilterFunction::Opacity(value))
            })
            .or_else(|_err: ParseError<'i>| {
                let value = FilterFunction::parse_argugment(
                    input,
                    |input| NonNegativeNumberOrPercentage::parse(context, input),
                    "saturate",
                    "1".into(),
                )?;
                Ok(FilterFunction::Saturate(value))
            })
            .or_else(|_err: ParseError<'i>| {
                let value = FilterFunction::parse_argugment(
                    input,
                    |input| NonNegativeNumberOrPercentage::parse(context, input),
                    "sepia",
                    "1".into(),
                )?;
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

#[derive(Clone)]
pub enum FilterFunctionOrUrl {
    Function(FilterFunction),
    Url(BrowserUrl),
}

impl FilterFunctionOrUrl {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        input
            .try_parse(|input| {
                let value = input.expect_url()?;
                let url = BrowserUrl::parse(&value).map_err(|_err| {
                    input.new_custom_error(StyleParseErrorKind::UnexpectedValue(value))
                })?;
                Ok(FilterFunctionOrUrl::Url(url))
            })
            .or_else(|_err: ParseError<'i>| {
                let function = FilterFunction::parse(context, input)?;
                Ok(FilterFunctionOrUrl::Function(function))
            })
    }
}

#[derive(Clone)]
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

pub fn parse_declared<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
    Filter::parse(context, input).map(PropertyDeclaration::Filter)
}
