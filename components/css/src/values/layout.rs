use cssparser::{match_ignore_ascii_case, Parser, ToCss, Token, _cssparser_internal_to_lowercase};

use super::CSSFloat;
use crate::parser::ParseError;
use crate::properties::declaration::property_keywords_impl;
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;

#[derive(Clone)]
#[repr(u8)]
pub enum BorderStyle {
    Hidden,
    None,
    Inset,
    Groove,
    Outset,
    Ridge,
    Dotted,
    Dashed,
    Solid,
    Double,
}

property_keywords_impl! { BorderStyle,
    BorderStyle::Hidden, "hidden",
    BorderStyle::None, "none",
    BorderStyle::Inset, "inset",
    BorderStyle::Groove, "groove",
    BorderStyle::Outset, "outset",
    BorderStyle::Ridge, "ridge",
    BorderStyle::Dotted, "dotted",
    BorderStyle::Dashed, "dashed",
    BorderStyle::Solid, "solid",
    BorderStyle::Double, "double",
}

#[derive(Clone)]
#[repr(u8)]
pub enum Overflow {
    Visible,
    Hidden,
    Scroll,
    Auto,
}

property_keywords_impl! { Overflow,
    Overflow::Visible, "visible",
    Overflow::Hidden, "hidden",
    Overflow::Scroll, "scroll",
    Overflow::Auto, "auto",
}

#[derive(Clone)]
#[repr(C)]
pub struct Size2D<L> {
    pub width: L,
    pub height: L,
}

/// A specified resolution.
#[derive(Clone, Debug, PartialEq)]
pub enum Resolution {
    /// Dots per inch.
    Dpi(CSSFloat),
    /// Dots per pixel.
    Dppx(CSSFloat),
    /// Dots per centimeter.
    Dpcm(CSSFloat),
}

impl Resolution {
    /// Parse a resolution.
    pub fn parse<'i, 't>(
        _context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        let token = input.next()?.clone();
        match token {
            cssparser::Token::Dimension { value, unit, .. } if value >= 0.0 => {
                match_ignore_ascii_case! { &unit,
                    "dpi" => Ok(Resolution::Dpi(value)),
                    "x" | "dppx"=> Ok(Resolution::Dppx(value)),
                    "dpcm" => Ok(Resolution::Dpcm(value)),
                    _ => return Err(input.new_custom_error(StyleParseErrorKind::UnexpectedDimension(unit))),
                }
            },
            ref t => return Err(input.new_unexpected_token_error(t.clone())),
        }
    }
}

impl ToCss for Resolution {
    fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
    where
        W: std::fmt::Write,
    {
        let (unit, value) = match self {
            Resolution::Dpi(value) => ("dpi", value),
            Resolution::Dppx(value) => ("dppx", value),
            Resolution::Dpcm(value) => ("dpcm", value),
        };
        dest.write_fmt(format_args!("{}{}", value, unit))
    }
}

#[derive(Clone)]
pub enum Box {
    BorderBox,
    PaddingBox,
    ContentBox,
}

property_keywords_impl! { Box,
    Box::BorderBox, "border-box",
    Box::PaddingBox, "padding-box",
    Box::ContentBox, "content-box",
}
