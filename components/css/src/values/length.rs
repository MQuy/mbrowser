use std::fmt::Write;

use cssparser::{
    CowRcStr, Parser, Token, _cssparser_internal_to_lowercase, match_ignore_ascii_case,
};

use super::number::NonNegative;
use super::percentage::Percentage;
use super::{AllowedNumericType, CSSFloat};
use crate::css_writer::ToCss;
use crate::parser::ParseError;
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;

/// An extension to `NoCalcLength` to parse `calc` expressions.

/// This is commonly used for the `<length>` values.
///
/// <https://drafts.csswg.org/css-values/#lengths>
#[derive(Clone, Debug, PartialEq)]
pub enum Length {
    /// The internal length type that cannot parse `calc`
    NoCalc(NoCalcLength),
}

impl Length {
    #[inline]
    pub fn parse_non_negative<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        Length::parse(context, input, AllowedNumericType::NonNegative)
    }

    pub fn parse<'i, 't>(
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

impl ToCss for Length {
    fn to_css<W>(&self, dest: &mut crate::css_writer::CssWriter<W>) -> core::fmt::Result
    where
        W: std::fmt::Write,
    {
        match self {
            Length::NoCalc(length) => length.to_css(dest),
        }
    }
}

/// A `<length>` without taking `calc` expressions into account
///
/// <https://drafts.csswg.org/css-values/#lengths>
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NoCalcLength {
    /// An absolute length
    ///
    /// <https://drafts.csswg.org/css-values/#absolute-length>
    Absolute(AbsoluteLength),

    /// A font-relative length:
    ///
    /// <https://drafts.csswg.org/css-values/#font-relative-lengths>
    FontRelative(FontRelativeLength),

    /// A viewport-relative length.
    ///
    /// <https://drafts.csswg.org/css-values/#viewport-relative-lengths>
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
    fn to_css<W>(&self, dest: &mut crate::css_writer::CssWriter<W>) -> core::fmt::Result
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
    fn to_css<W>(&self, dest: &mut crate::css_writer::CssWriter<W>) -> core::fmt::Result
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

/// A font relative length.
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
    fn to_css<W>(&self, dest: &mut crate::css_writer::CssWriter<W>) -> core::fmt::Result
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

/// A viewport-relative length.
///
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
    fn to_css<W>(&self, dest: &mut crate::css_writer::CssWriter<W>) -> core::fmt::Result
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

#[derive(Clone)]
pub enum LengthPercentage {
    Length(NoCalcLength),
    Percentage(Percentage),
    // Calc(Box<CalcLengthPercentage>),
}

#[derive(Clone)]
pub enum GenericLengthPercentageOrAuto<LengthPercent> {
    LengthPercentage(LengthPercent),
    Auto,
}

pub type LengthPercentageOrAuto = GenericLengthPercentageOrAuto<LengthPercentage>;
pub type NonNegativeLength = NonNegative<Length>;

#[derive(Clone)]
#[repr(C, u8)]
pub enum GenericSize<LengthPercent> {
    LengthPercentage(LengthPercent),
    Auto,
}

pub type NonNegativeLengthPercentage = NonNegative<LengthPercentage>;
pub type Size = GenericSize<NonNegativeLengthPercentage>;

#[derive(Clone)]
#[repr(C, u8)]
pub enum GenericMaxSize<LengthPercent> {
    LengthPercentage(LengthPercent),
    None,
}
pub type MaxSize = GenericMaxSize<NonNegativeLengthPercentage>;

#[derive(Clone)]
#[repr(C, u8)]
pub enum GenericLengthPercentageOrNormal<LengthPercent> {
    LengthPercentage(LengthPercent),
    Normal,
}
pub type NonNegativeLengthPercentageOrNormal =
    GenericLengthPercentageOrNormal<NonNegativeLengthPercentage>;

pub type NonNegativeLengthOrAuto = GenericLengthPercentageOrAuto<NonNegativeLength>;
