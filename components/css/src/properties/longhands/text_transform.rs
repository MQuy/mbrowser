use cssparser::Parser;

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;

#[derive(Clone)]
#[repr(C)]
pub enum TextTransformCase {
    None,
    Uppercase,
    Lowercase,
    Capitalize,
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

pub fn parse<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<TextTransform, ParseError<'i>> {
    todo!()
}

pub fn parse_declared<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
    parse(context, input).map(PropertyDeclaration::TextTransform)
}
