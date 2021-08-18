use cssparser::{match_ignore_ascii_case, Parser, ToCss, Token, _cssparser_internal_to_lowercase};

use crate::parser::{parse_in_any_order, parse_item_if_missing, ParseError};
use crate::properties::declaration::{property_keywords_impl, PropertyDeclaration};
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;

#[derive(Clone, PartialEq)]
#[repr(C)]
pub enum TextTransformCase {
    Uppercase,
    Lowercase,
    Capitalize,
}

property_keywords_impl! { TextTransformCase,
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
    case: Option<TextTransformCase>,
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
        input
            .try_parse(|input| {
                input.expect_ident_matching("none")?;
                Ok(TextTransform::None)
            })
            .or_else(|_err: ParseError<'i>| {
                let mut case = None;
                let mut bits = None;
                parse_in_any_order(
                    input,
                    &mut [
                        &mut |input| {
                            parse_item_if_missing(input, &mut case, |_, input| {
                                TextTransformCase::parse(input)
                            })
                        },
                        &mut |input| {
                            parse_item_if_missing(input, &mut bits, |size, input| {
                                let location = input.current_source_location();
                                let ident = input.expect_ident()?;
                                let value = size.map_or(0, |size| size);
                                Ok(match_ignore_ascii_case! {ident,
                                    "full-width" => value | FULL_WIDTH,
                                    "full-size-kana" => value | FULL_SIZE_KANA,
                                    _ => return Err(location.new_custom_error(StyleParseErrorKind::UnexpectedValue(ident.clone())))
                                })
                            })
                        },
                    ],
                );
                if case.is_none() && bits.is_none() {
                    Err(input.new_custom_error(StyleParseErrorKind::UnspecifiedError))
                } else {
                    let bits = bits.map_or(0, |size| size);
                    Ok(TextTransform::Transform(TextTransformValue { case, size: TextTransformSize { bits }}))
                }
            })
    }
}

pub fn parse_declared<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
    TextTransform::parse(context, input).map(PropertyDeclaration::TextTransform)
}
