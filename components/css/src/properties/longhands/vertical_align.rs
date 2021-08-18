use cssparser::{match_ignore_ascii_case, Parser, ToCss, Token, _cssparser_internal_to_lowercase};

use crate::parser::ParseError;
use crate::properties::declaration::{property_keywords_impl, PropertyDeclaration};
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::length::LengthPercentage;

#[derive(Clone)]
#[repr(u8)]
pub enum VerticalAlignKeyword {
    Baseline,
    Sub,
    Super,
    Top,
    TextTop,
    Middle,
    Bottom,
    TextBottom,
}

property_keywords_impl! { VerticalAlignKeyword,
    VerticalAlignKeyword::Baseline, "baseline",
    VerticalAlignKeyword::Sub, "sub",
    VerticalAlignKeyword::Super, "super",
    VerticalAlignKeyword::Top, "top",
    VerticalAlignKeyword::TextTop, "text-top",
    VerticalAlignKeyword::Middle, "middle",
    VerticalAlignKeyword::Bottom, "bottom",
    VerticalAlignKeyword::TextBottom, "text-bottom",
}

#[derive(Clone)]
pub enum VerticalAlign {
    VerticalAlignKeyword,
    LengthPercentage(LengthPercentage),
}

impl VerticalAlign {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<VerticalAlign, ParseError<'i>> {
        todo!()
    }
}

pub fn parse_declared<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
    VerticalAlign::parse(context, input).map(PropertyDeclaration::VerticalAlign)
}
