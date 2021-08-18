use cssparser::{match_ignore_ascii_case, Parser, _cssparser_internal_to_lowercase};

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::length::LengthPercentage;

#[derive(Clone)]
pub enum TextOverflowSide {
    Clip,
    Ellipsis,
    String(String),
    Fade(Option<LengthPercentage>),
}

impl TextOverflowSide {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        input.try_parse(|input| {
            let location = input.current_source_location();
            let ident = input.expect_ident()?;
            Ok(match_ignore_ascii_case! { ident,
                "clip" => TextOverflowSide::Clip,
                "ellipsis" => TextOverflowSide::Ellipsis,
                "fade" => TextOverflowSide::Fade(None),
                _ => return Err(location.new_custom_error(StyleParseErrorKind::UnexpectedValue(ident.clone())))
            })
        }).or_else(|_err: ParseError<'i>| {
            let value = input.expect_string()?.to_string();
            Ok(TextOverflowSide::String(value))
        }).or_else(|_err: ParseError<'i>| {
            input.expect_function_matching("fade")?;
            let arg = input.parse_nested_block(|input| {
                LengthPercentage::parse(context, input)
            })?;
            Ok(TextOverflowSide::Fade(Some(arg)))
        })
    }
}

#[derive(Clone)]
pub struct TextOverflow {
    first: TextOverflowSide,
    second: Option<TextOverflowSide>,
}

impl TextOverflow {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<TextOverflow, ParseError<'i>> {
        let first = TextOverflowSide::parse(context, input)?;
        let second = TextOverflowSide::parse(context, input).ok();
        Ok(TextOverflow { first, second })
    }
}

pub fn parse_declared<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
    TextOverflow::parse(context, input).map(PropertyDeclaration::TextOverflow)
}
