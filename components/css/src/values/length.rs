use std::fmt::Write;

use cssparser::{
    CowRcStr, Parser, ToCss, Token, _cssparser_internal_to_lowercase, match_ignore_ascii_case,
};
use regex::Regex;

use super::generics::number::NonNegative;
use super::number::NonNegativeNumber;
use super::percentage::Percentage;
use super::{AllowedNumericType, CSSFloat};
use crate::parser::ParseError;
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;

/// <https://drafts.csswg.org/css-values/#lengths>
#[derive(Clone, Debug, PartialEq)]
pub enum Length {
    NoCalc(NoCalcLength),
}

impl Length {
    #[inline]
    pub fn parse_non_negative<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        Length::parse_internal(context, input, AllowedNumericType::NonNegative)
    }

    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        Length::parse_internal(context, input, AllowedNumericType::All)
    }

    pub fn parse_internal<'i, 't>(
        _context: &ParserContext,
        input: &mut Parser<'i, 't>,
        num_context: AllowedNumericType,
    ) -> Result<Self, ParseError<'i>> {
        let location = input.current_source_location();
        let token = input.next()?.clone();
        match token {
            Token::Dimension {
                value, ref unit, ..
            } if num_context.is_ok(value) => NoCalcLength::parse(unit, value)
                .map(Length::NoCalc)
                .map_err(|()| location.new_unexpected_token_error(token.clone())),
            Token::Number { value, .. } if num_context.is_ok(value) => {
                if value != 0.0 {
                    return Err(location.new_custom_error(StyleParseErrorKind::UnspecifiedError));
                }
                Ok(Length::NoCalc(NoCalcLength::Absolute(AbsoluteLength::Px(
                    value,
                ))))
            },
            ref t => return Err(location.new_unexpected_token_error(t.clone())),
        }
    }
}

impl From<&str> for Length {
    fn from(text: &str) -> Self {
        let regex = Regex::new(r"/px|in|cm|mm|q|pt|pc|em|ex|ch|vw|vh|vmin|vmax/i").unwrap();
        let index = regex.find(text).unwrap().start();
        let (value, unit) = (&text[..index], &text[index..]);
        let value = value.parse::<f32>().unwrap();
        Length::NoCalc(match_ignore_ascii_case! { unit,
            "px" => NoCalcLength::Absolute(AbsoluteLength::Px(value)),
            "in" => NoCalcLength::Absolute(AbsoluteLength::In(value)),
            "cm" => NoCalcLength::Absolute(AbsoluteLength::Cm(value)),
            "mm" => NoCalcLength::Absolute(AbsoluteLength::Mm(value)),
            "q" => NoCalcLength::Absolute(AbsoluteLength::Q(value)),
            "pt" => NoCalcLength::Absolute(AbsoluteLength::Pt(value)),
            "pc" => NoCalcLength::Absolute(AbsoluteLength::Pc(value)),
            "em" => NoCalcLength::FontRelative(FontRelativeLength::Em(value)),
            "ex" => NoCalcLength::FontRelative(FontRelativeLength::Ex(value)),
            "ch" => NoCalcLength::FontRelative(FontRelativeLength::Ch(value)),
            "vw" => NoCalcLength::ViewportPercentage(ViewportPercentageLength::Vw(value)),
            "vh" => NoCalcLength::ViewportPercentage(ViewportPercentageLength::Vh(value)),
            "vmin" => NoCalcLength::ViewportPercentage(ViewportPercentageLength::Vmin(value)),
            "vmax" => NoCalcLength::ViewportPercentage(ViewportPercentageLength::Vmax(value)),
            _ => panic!("unit {} is not supported", unit),
        })
    }
}

impl ToCss for Length {
    fn to_css<W>(&self, dest: &mut W) -> core::fmt::Result
    where
        W: std::fmt::Write,
    {
        match self {
            Length::NoCalc(length) => length.to_css(dest),
        }
    }
}

/// <https://drafts.csswg.org/css-values/#lengths>
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NoCalcLength {
    Absolute(AbsoluteLength),
    FontRelative(FontRelativeLength),
    ViewportPercentage(ViewportPercentageLength),
}

impl NoCalcLength {
    pub fn parse(unit: &CowRcStr, value: CSSFloat) -> Result<Self, ()> {
        Ok(match_ignore_ascii_case! { &unit,
            "px" => NoCalcLength::Absolute(AbsoluteLength::Px(value)),
            "in" => NoCalcLength::Absolute(AbsoluteLength::In(value)),
            "cm" => NoCalcLength::Absolute(AbsoluteLength::Cm(value)),
            "mm" => NoCalcLength::Absolute(AbsoluteLength::Mm(value)),
            "q" => NoCalcLength::Absolute(AbsoluteLength::Q(value)),
            "pt" => NoCalcLength::Absolute(AbsoluteLength::Pt(value)),
            "pc" => NoCalcLength::Absolute(AbsoluteLength::Pc(value)),
            "em" => NoCalcLength::FontRelative(FontRelativeLength::Em(value)),
            "ex" => NoCalcLength::FontRelative(FontRelativeLength::Ex(value)),
            "ch" => NoCalcLength::FontRelative(FontRelativeLength::Ch(value)),
            "vw" => NoCalcLength::ViewportPercentage(ViewportPercentageLength::Vw(value)),
            "vh" => NoCalcLength::ViewportPercentage(ViewportPercentageLength::Vh(value)),
            "vmin" => NoCalcLength::ViewportPercentage(ViewportPercentageLength::Vmin(value)),
            "vmax" => NoCalcLength::ViewportPercentage(ViewportPercentageLength::Vmax(value)),
            _ => return Err(()),
        })
    }
}

impl ToCss for NoCalcLength {
    fn to_css<W>(&self, dest: &mut W) -> core::fmt::Result
    where
        W: std::fmt::Write,
    {
        match self {
            NoCalcLength::Absolute(absolute) => absolute.to_css(dest),
            NoCalcLength::FontRelative(font) => font.to_css(dest),
            NoCalcLength::ViewportPercentage(viewport) => viewport.to_css(dest),
        }
    }
}

/// <https://drafts.csswg.org/css-values/#absolute-length>
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AbsoluteLength {
    /// An absolute length in pixels (px)
    Px(CSSFloat),
    /// An absolute length in inches (in)
    In(CSSFloat),
    /// An absolute length in centimeters (cm)
    Cm(CSSFloat),
    /// An absolute length in millimeters (mm)
    Mm(CSSFloat),
    /// An absolute length in quarter-millimeters (q)
    Q(CSSFloat),
    /// An absolute length in points (pt)
    Pt(CSSFloat),
    /// An absolute length in pica (pc)
    Pc(CSSFloat),
}

impl ToCss for AbsoluteLength {
    fn to_css<W>(&self, dest: &mut W) -> core::fmt::Result
    where
        W: std::fmt::Write,
    {
        let (unit, value) = match self {
            AbsoluteLength::Px(value) => ("px", value),
            AbsoluteLength::In(value) => ("in", value),
            AbsoluteLength::Cm(value) => ("cm", value),
            AbsoluteLength::Mm(value) => ("mm", value),
            AbsoluteLength::Q(value) => ("q", value),
            AbsoluteLength::Pt(value) => ("pt", value),
            AbsoluteLength::Pc(value) => ("pc", value),
        };
        dest.write_fmt(format_args!("{}{}", value, unit))
    }
}

/// <https://drafts.csswg.org/css-values/#font-relative-lengths>
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FontRelativeLength {
    /// A "em" value: https://drafts.csswg.org/css-values/#em
    Em(CSSFloat),
    /// A "ex" value: https://drafts.csswg.org/css-values/#ex
    Ex(CSSFloat),
    /// A "ch" value: https://drafts.csswg.org/css-values/#ch
    Ch(CSSFloat),
    /// A "rem" value: https://drafts.csswg.org/css-values/#rem
    Rem(CSSFloat),
}

impl ToCss for FontRelativeLength {
    fn to_css<W>(&self, dest: &mut W) -> core::fmt::Result
    where
        W: Write,
    {
        let (unit, value) = match self {
            FontRelativeLength::Em(value) => ("em", value),
            FontRelativeLength::Ex(value) => ("ex", value),
            FontRelativeLength::Ch(value) => ("ch", value),
            FontRelativeLength::Rem(value) => ("rem", value),
        };
        dest.write_fmt(format_args!("{}{}", value, unit))
    }
}

/// <https://drafts.csswg.org/css-values/#viewport-relative-lengths>
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ViewportPercentageLength {
    /// A vw unit: https://drafts.csswg.org/css-values/#vw
    Vw(CSSFloat),
    /// A vh unit: https://drafts.csswg.org/css-values/#vh
    Vh(CSSFloat),
    /// <https://drafts.csswg.org/css-values/#vmin>
    Vmin(CSSFloat),
    /// <https://drafts.csswg.org/css-values/#vmax>
    Vmax(CSSFloat),
}

impl ToCss for ViewportPercentageLength {
    fn to_css<W>(&self, dest: &mut W) -> core::fmt::Result
    where
        W: Write,
    {
        let (unit, value) = match self {
            ViewportPercentageLength::Vw(value) => ("vw", value),
            ViewportPercentageLength::Vh(value) => ("vh", value),
            ViewportPercentageLength::Vmin(value) => ("vmin", value),
            ViewportPercentageLength::Vmax(value) => ("vmax", value),
        };
        dest.write_fmt(format_args!("{}{}", value, unit))
    }
}

/// https://www.w3.org/TR/css-values-4/#typedef-length-percentage
/// <length-percentage> = [ <length> | <percentage> ]
#[derive(Clone)]
pub enum LengthPercentage {
    Length(Length),
    Percentage(Percentage),
}

impl LengthPercentage {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        Self::parse_internal(context, input, AllowedNumericType::All)
    }

    pub fn parse_non_negative<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        Self::parse_internal(context, input, AllowedNumericType::NonNegative)
    }

    pub fn parse_internal<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
        num_context: AllowedNumericType,
    ) -> Result<Self, ParseError<'i>> {
        input
            .try_parse(|input| {
                let length = Length::parse_internal(context, input, num_context)?;
                Ok(LengthPercentage::Length(length))
            })
            .or_else(|_err: ParseError<'i>| {
                let percentage = Percentage::parse(context, input)?;
                Ok(LengthPercentage::Percentage(percentage))
            })
    }
}

impl From<&str> for LengthPercentage {
    fn from(text: &str) -> Self {
        match text.find(|ch| ch == '%') {
            Some(index) => {
                let value = text[..index].parse::<f32>().unwrap();
                LengthPercentage::Percentage(Percentage::new(value))
            },
            None => LengthPercentage::Length(text.into()),
        }
    }
}

impl ToCss for LengthPercentage {
    fn to_css<W>(&self, dest: &mut W) -> core::fmt::Result
    where
        W: Write,
    {
        todo!()
    }
}

/// value = <length [0, ∞]>
pub type NonNegativeLength = NonNegative<Length>;

impl NonNegativeLength {
    pub fn new(value: Length) -> Self {
        NonNegative::<Length>(value)
    }

    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        let length = Length::parse_non_negative(context, input)?;
        Ok(Self(length))
    }
}

impl From<&str> for NonNegativeLength {
    fn from(text: &str) -> Self {
        assert!(text.chars().nth(0).unwrap() != '-');
        NonNegativeLength::new(text.into())
    }
}

/// value = <length [0, ∞]> | <percentage>
pub type NonNegativeLengthPercentage = NonNegative<LengthPercentage>;

impl NonNegativeLengthPercentage {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        let length_percentage = LengthPercentage::parse_non_negative(context, input)?;
        Ok(Self(length_percentage))
    }
}

/// Generic for Length/Auto
#[derive(Clone)]
pub enum GenericLengthOrAuto<Length> {
    Length(Length),
    Auto,
}

impl<L> GenericLengthOrAuto<L> {
    pub fn parse_with<'i, 't, LP>(
        input: &mut Parser<'i, 't>,
        length_parser: LP,
    ) -> Result<Self, ParseError<'i>>
    where
        LP: FnOnce(&mut Parser<'i, 't>) -> Result<L, ParseError<'i>>,
    {
        input
            .try_parse(|input| {
                input.expect_ident_matching("auto")?;
                Ok(Self::Auto)
            })
            .or_else(|_err: ParseError<'i>| {
                let length = length_parser(input)?;
                Ok(Self::Length(length))
            })
    }
}

/// Generic for Length/None
#[derive(Clone)]
pub enum GenericLengthOrNone<Length> {
    Length(Length),
    None,
}

impl<L> GenericLengthOrNone<L> {
    pub fn parse_with<'i, 't, LP>(
        input: &mut Parser<'i, 't>,
        length_parser: LP,
    ) -> Result<Self, ParseError<'i>>
    where
        LP: FnOnce(&mut Parser<'i, 't>) -> Result<L, ParseError<'i>>,
    {
        input
            .try_parse(|input| {
                input.expect_ident_matching("none")?;
                Ok(Self::None)
            })
            .or_else(|_err: ParseError<'i>| {
                let length = length_parser(input)?;
                Ok(Self::Length(length))
            })
    }
}

/// value = <length> | none
pub type NonNegativeLengthOrNone = GenericLengthOrNone<NonNegativeLength>;

impl NonNegativeLengthOrNone {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        Self::parse_with(input, |input| NonNegativeLength::parse(context, input))
    }
}

/// Generic for Length/Percentage/Auto
#[derive(Clone)]
pub enum GenericLengthPercentageOrAuto<LengthPercent> {
    LengthPercentage(LengthPercent),
    Auto,
}

impl<LP> GenericLengthPercentageOrAuto<LP> {
    pub fn parse_with<'i, 't, LPP>(
        input: &mut Parser<'i, 't>,
        length_percentage_parser: LPP,
    ) -> Result<Self, ParseError<'i>>
    where
        LPP: FnOnce(&mut Parser<'i, 't>) -> Result<LP, ParseError<'i>>,
    {
        input
            .try_parse(|input| {
                input.expect_ident_matching("auto")?;
                Ok(Self::Auto)
            })
            .or_else(|_err: ParseError<'i>| {
                let length_percentage = length_percentage_parser(input)?;
                Ok(Self::LengthPercentage(length_percentage))
            })
    }
}

/// value = <length> | <percentage> | auto
pub type LengthPercentageOrAuto = GenericLengthPercentageOrAuto<LengthPercentage>;

impl LengthPercentageOrAuto {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        Self::parse_with(input, |input| LengthPercentage::parse(context, input))
    }
}

impl ToCss for LengthPercentageOrAuto {
    fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
    where
        W: std::fmt::Write,
    {
        todo!()
    }
}

pub type NonNegativeLengthOrAuto = GenericLengthPercentageOrAuto<NonNegativeLength>;

impl NonNegativeLengthOrAuto {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        Self::parse_with(input, |input| NonNegativeLength::parse(context, input))
    }
}

/// Generic for Length/Percentage/Number/Auto
#[derive(Clone)]
pub enum GenericLengthPercentageNumberOrAuto<LengthPercent, Number> {
    LengthPercentage(LengthPercent),
    Number(Number),
    Auto,
}

impl<LP, N> GenericLengthPercentageNumberOrAuto<LP, N> {
    pub fn parse_with<'i, 't, LPP, NP>(
        input: &mut Parser<'i, 't>,
        length_percentage_parser: LPP,
        number_parser: NP,
    ) -> Result<Self, ParseError<'i>>
    where
        LPP: FnOnce(&mut Parser<'i, 't>) -> Result<LP, ParseError<'i>>,
        NP: FnOnce(&mut Parser<'i, 't>) -> Result<N, ParseError<'i>>,
    {
        input
            .try_parse(|input| {
                input.expect_ident_matching("auto")?;
                Ok(Self::Auto)
            })
            .or_else(|_err: ParseError<'i>| {
                input
                    .try_parse(|input| {
                        let length_percentage = length_percentage_parser(input)?;
                        Ok(Self::LengthPercentage(length_percentage))
                    })
                    .or_else(|_err: ParseError<'i>| {
                        let number = number_parser(input)?;
                        Ok(Self::Number(number))
                    })
            })
    }
}

/// value = <length [0, ∞]> | <percentage> | <number [0, ∞]> | auto
pub type NonNegativeLengthPercentageNumberOrAuto =
    GenericLengthPercentageNumberOrAuto<NonNegativeLength, NonNegativeNumber>;

impl NonNegativeLengthPercentageNumberOrAuto {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        Self::parse_with(
            input,
            |input| NonNegativeLength::parse(context, input),
            |input| NonNegativeNumber::parse(context, input),
        )
    }
}

#[derive(Clone)]
pub enum ExtremumLength<LengthPercent> {
    MaxContent,
    MinContent,
    FitContent(LengthPercent),
}

#[derive(Clone)]
pub enum GenericSize<LengthPercent> {
    Auto,
    LengthPercentage(LengthPercent),
    ExtremumLength(ExtremumLength<LengthPercent>),
}

impl<LP> GenericSize<LP> {
    pub fn parse_with<'i, 't, F>(
        input: &mut Parser<'i, 't>,
        item_parser: F,
    ) -> Result<Self, ParseError<'i>>
    where
        F: Fn(&mut Parser<'i, 't>) -> Result<LP, ParseError<'i>>,
    {
        input
            .try_parse(|input| {
                input.expect_ident_matching("auto")?;
                Ok(Self::Auto)
            })
            .or_else(|_err: ParseError<'i>| {
                input
                    .try_parse(|input| {
                        let length_percentage = item_parser(input)?;
                        Ok(Self::LengthPercentage(length_percentage))
                    })
                    .or_else(|_err: ParseError<'i>| {
                        let location = input.current_source_location();
                        let ident = input.expect_ident()?;
                        Ok(Self::ExtremumLength(match_ignore_ascii_case! { ident,
                            "max-content" => ExtremumLength::MaxContent,
                            "min-content" => ExtremumLength::MinContent,
                            "fit-content" => {
                                let length_percentage = item_parser(input)?;
                                ExtremumLength::FitContent(length_percentage)
                            },
                            _ => return Err(location.new_custom_error(StyleParseErrorKind::UnexpectedValue(ident.clone())))
                        }))
                    })
            })
    }
}

pub type Size = GenericSize<NonNegativeLengthPercentage>;

impl Size {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        Self::parse_with(input, |input| {
            NonNegativeLengthPercentage::parse(context, input)
        })
    }
}

impl ToCss for Size {
    fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
    where
        W: std::fmt::Write,
    {
        todo!()
    }
}

#[derive(Clone)]
#[repr(C, u8)]
pub enum GenericLengthPercentageOrNormal<LengthPercent> {
    LengthPercentage(LengthPercent),
    Normal,
}

impl<LP> GenericLengthPercentageOrNormal<LP> {
    pub fn parse_with<'i, 't, F>(
        input: &mut Parser<'i, 't>,
        item_parser: F,
    ) -> Result<Self, ParseError<'i>>
    where
        F: Fn(&mut Parser<'i, 't>) -> Result<LP, ParseError<'i>>,
    {
        input
            .try_parse(|input| {
                input.expect_ident_matching("normal")?;
                Ok(Self::Normal)
            })
            .or_else(|_err: ParseError<'i>| {
                let length_percent = item_parser(input)?;
                Ok(Self::LengthPercentage(length_percent))
            })
    }
}

pub type LengthPercentageOrNormal = GenericLengthPercentageOrNormal<LengthPercentage>;

impl LengthPercentageOrNormal {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        Self::parse_with(input, |input| LengthPercentage::parse(context, input))
    }
}

pub type NonNegativeLengthPercentageOrNormal =
    GenericLengthPercentageOrNormal<NonNegativeLengthPercentage>;

impl NonNegativeLengthPercentageOrNormal {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        Self::parse_with(input, |input| {
            NonNegativeLengthPercentage::parse(context, input)
        })
    }
}

#[derive(Clone)]
#[repr(C, u8)]
pub enum GenericLengthPercentageNumberOrNormal<Number, LengthPercent> {
    LengthPercentage(LengthPercent),
    Number(Number),
    Normal,
}

impl<N, LP> GenericLengthPercentageNumberOrNormal<N, LP> {
    pub fn parse_with<'i, 't, NP, LPP>(
        input: &mut Parser<'i, 't>,
        number_parser: NP,
        length_percentage_parser: LPP,
    ) -> Result<Self, ParseError<'i>>
    where
        NP: Fn(&mut Parser<'i, 't>) -> Result<N, ParseError<'i>>,
        LPP: Fn(&mut Parser<'i, 't>) -> Result<LP, ParseError<'i>>,
    {
        input
            .try_parse(|input| {
                input.expect_ident_matching("normal")?;
                Ok(Self::Normal)
            })
            .or_else(|_err: ParseError<'i>| {
                let number = number_parser(input)?;
                Ok(Self::Number(number))
            })
            .or_else(|_err: ParseError<'i>| {
                let length_percent = length_percentage_parser(input)?;
                Ok(Self::LengthPercentage(length_percent))
            })
    }
}

pub type NonNegativeLengthPercentageNumberOrNormal =
    GenericLengthPercentageNumberOrNormal<NonNegativeNumber, NonNegativeLengthPercentage>;

impl NonNegativeLengthPercentageNumberOrNormal {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        Self::parse_with(
            input,
            |input| NonNegativeNumber::parse(context, input),
            |input| NonNegativeLengthPercentage::parse(context, input),
        )
    }
}

#[derive(Clone)]
pub enum GenericLengthOrNumber<N, L> {
    Number(N),
    Length(L),
}

impl<N, L> GenericLengthOrNumber<N, L> {
    pub fn parse_with<'i, 't, LP, NP>(
        input: &mut Parser<'i, 't>,
        number_parser: NP,
        length_parser: LP,
    ) -> Result<Self, ParseError<'i>>
    where
        NP: FnOnce(&mut Parser<'i, 't>) -> Result<N, ParseError<'i>>,
        LP: FnOnce(&mut Parser<'i, 't>) -> Result<L, ParseError<'i>>,
    {
        input
            .try_parse(|input| {
                let number = number_parser(input)?;
                Ok(Self::Number(number))
            })
            .or_else(|_err: ParseError<'i>| {
                let length = length_parser(input)?;
                Ok(Self::Length(length))
            })
    }
}

pub type NonNegativeLengthOrNumber = GenericLengthOrNumber<NonNegativeNumber, NonNegativeLength>;

impl NonNegativeLengthOrNumber {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        Self::parse_with(
            input,
            |input| NonNegativeNumber::parse(context, input),
            |input| NonNegativeLength::parse(context, input),
        )
    }
}

#[derive(Clone)]
pub struct Rect<T>(pub T, pub T, pub T, pub T)
where
    T: Clone;

impl<T: Clone> Rect<T> {
    pub fn parse_with<'i, 't, F>(
        input: &mut Parser<'i, 't>,
        item_parser: F,
    ) -> Result<Self, ParseError<'i>>
    where
        F: Fn(&mut Parser<'i, 't>) -> Result<T, ParseError<'i>>,
    {
        let first = item_parser(input)?;
        let second = if let Ok(second) = item_parser(input) {
            second
        } else {
            return Ok(Self(first.clone(), first.clone(), first.clone(), first));
        };
        let third = if let Ok(third) = item_parser(input) {
            third
        } else {
            return Ok(Self(first.clone(), second.clone(), first, second));
        };
        let forth = if let Ok(forth) = item_parser(input) {
            forth
        } else {
            return Ok(Self(first, second.clone(), third, second));
        };
        Ok(Self(first, second, third, forth))
    }
}

pub type NonNegativeLengthOrNumberRect = Rect<NonNegativeLengthOrNumber>;

impl NonNegativeLengthOrNumberRect {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        Rect::parse_with(input, |input| {
            NonNegativeLengthOrNumber::parse(context, input)
        })
    }
}

#[derive(Clone)]
pub enum GenericRectOrAuto<T: Clone> {
    Auto,
    Rect(Rect<T>),
}

impl<T: Clone> GenericRectOrAuto<T> {
    pub fn parse_with<'i, 't, F>(
        input: &mut Parser<'i, 't>,
        item_parser: F,
    ) -> Result<Self, ParseError<'i>>
    where
        F: Fn(&mut Parser<'i, 't>) -> Result<T, ParseError<'i>>,
    {
        input
            .try_parse(|input| {
                input.expect_ident_matching("auto")?;
                Ok(Self::Auto)
            })
            .or_else(|_err: ParseError<'i>| {
                let rect = Rect::parse_with(input, item_parser)?;
                Ok(Self::Rect(rect))
            })
    }
}

pub type LengthOrAutoRectAuto = GenericRectOrAuto<GenericLengthOrAuto<Length>>;

impl LengthOrAutoRectAuto {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        Self::parse_with(input, |input| {
            GenericLengthOrAuto::<Length>::parse_with(input, |input| Length::parse(context, input))
        })
    }
}

#[derive(Clone)]
pub struct Pair<T>(pub T, pub T);

impl<T: Clone> Pair<T> {
    pub fn new(left: T, right: T) -> Pair<T> {
        Pair::<T>(left, right)
    }

    pub fn parse_with<'i, 't, F>(
        input: &mut Parser<'i, 't>,
        item_parser: F,
    ) -> Result<Self, ParseError<'i>>
    where
        F: Fn(&mut Parser<'i, 't>) -> Result<T, ParseError<'i>>,
    {
        let first = item_parser(input)?;
        let second = item_parser(input);
        if let Ok(second) = second {
            Ok(Self(first, second))
        } else {
            Ok(Self(first.clone(), first))
        }
    }
}

impl<T: ToCss> ToCss for Pair<T> {
    fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
    where
        W: std::fmt::Write,
    {
        self.0.to_css(dest)?;
        dest.write_char(' ')?;
        self.1.to_css(dest)
    }
}
