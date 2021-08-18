use cssparser::{match_ignore_ascii_case, Parser, ToCss, Token, _cssparser_internal_to_lowercase};

use crate::parser::ParseError;
use crate::properties::declaration::{property_keywords_impl, PropertyDeclaration};
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;

#[derive(Clone)]
#[repr(C)]
pub enum TextTransformCase {
    None,
    Uppercase,
    Lowercase,
    Capitalize,
}

property_keywords_impl! { TextTransformCase,
    TextTransformCase::None, "none",
    TextTransformCase::Uppercase, "uppercase",
    TextTransformCase::Lowercase, "lowercase",
    TextTransformCase::Capitalize, "capitalize",
}

#[derive(Clone)]
#[repr(C)]
pub struct TextTransformSize {
    bits: u8,
}

pub const FULL_WIDTH: u8 = 1 << 0;
pub const FULL_SIZE_KANA: u8 = 1 << 1;

#[derive(Clone)]
#[repr(C)]
pub struct TextTransformValue {
    case: TextTransformCase,
    size: TextTransformSize,
}

#[derive(Clone)]
#[repr(C)]
pub enum TextTransform {
    None,
    Transform(TextTransformValue),
}

impl TextTransform {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<TextTransform, ParseError<'i>> {
        todo!()
    }
}

pub fn parse_declared<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
    TextTransform::parse(context, input).map(PropertyDeclaration::TextTransform)
}
