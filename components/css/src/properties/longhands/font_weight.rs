use cssparser::{match_ignore_ascii_case, Parser, _cssparser_internal_to_lowercase};

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::number::NonNegativeNumber;

#[derive(Clone)]
pub enum AbsoluteFontWeight {
    Weight(NonNegativeNumber),
    Normal,
    Bold,
}

impl AbsoluteFontWeight {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        input.try_parse(|input| {
            let location = input.current_source_location();
            let ident = input.expect_ident()?;
            Ok(match_ignore_ascii_case! { ident,
                "normal" => AbsoluteFontWeight::Normal,
                "bold" => AbsoluteFontWeight::Bold,
                _ => {
                    return Err(
                        location.new_custom_error(StyleParseErrorKind::UnexpectedValue(ident.clone()))
                    )
                },
            })
            .or_else(|_err: ParseError<'i>| {
                let value = NonNegativeNumber::parse_in_range(
                    context,
                    input,
                    0.0,
                    1000.0,
                )?;
                Ok(AbsoluteFontWeight::Weight(value))
            })
        })
    }
}

#[derive(Clone)]
pub enum FontWeight {
    Absolute(AbsoluteFontWeight),
    Bolder,
    Lighter,
}

impl FontWeight {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        input.try_parse(|input| {
            let location = input.current_source_location();
            let ident = input.expect_ident()?;
            Ok(match_ignore_ascii_case! { ident,
                "bolder" => FontWeight::Bolder,
                "lighter" => FontWeight::Lighter,
                _ => {
                    return Err(
                        location.new_custom_error(StyleParseErrorKind::UnexpectedValue(ident.clone()))
                    )
                },
            })
            .or_else(|_err: ParseError<'i>| {
                let value = AbsoluteFontWeight::parse(context, input)?;
                Ok(FontWeight::Absolute(value))
            })
        })
    }
}

pub fn parse_declared<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
    FontWeight::parse(context, input).map(PropertyDeclaration::FontWeight)
}
