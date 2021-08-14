use cssparser::{match_ignore_ascii_case, Parser, _cssparser_internal_to_lowercase};

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::length::LengthPercentageOrAuto;

#[derive(Clone)]
pub enum BackgroundSize {
    ExplicitSize {
        width: LengthPercentageOrAuto,
        height: LengthPercentageOrAuto,
    },
    Cover,
    Contain,
}

impl BackgroundSize {
    /// https://drafts.csswg.org/css-backgrounds/#background-size
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        input
            .try_parse(|input| {
                let width = LengthPercentageOrAuto::parse(context, input)?;
                let height = LengthPercentageOrAuto::parse(context, input);

                Ok(if let Ok(height) = height {
                    BackgroundSize::ExplicitSize { width, height }
                } else {
                    BackgroundSize::ExplicitSize {
                        width,
                        height: LengthPercentageOrAuto::Auto,
                    }
                })
            })
            .or_else(|err: ParseError<'i>| {
                let location = input.current_source_location();
                let ident = input.expect_ident()?;
                Ok(match_ignore_ascii_case! { ident,
                    "cover" => BackgroundSize::Cover,
                    "contain" => BackgroundSize::Contain,
                    _ => return Err(location.new_custom_error(StyleParseErrorKind::UnexpectedValue(ident.clone())))
                })
            })
    }
}

pub fn parse_declared<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
    BackgroundSize::parse(context, input).map(PropertyDeclaration::BackgroundSize)
}
