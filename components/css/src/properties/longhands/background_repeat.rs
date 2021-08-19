use cssparser::{match_ignore_ascii_case, Parser, ToCss, Token, _cssparser_internal_to_lowercase};

use crate::parser::ParseError;
use crate::properties::declaration::{property_keywords_impl, PropertyDeclaration};
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::length::Pair;

#[derive(Clone)]
pub enum BackgroundRepeatKeyword {
    Repeat,
    Space,
    Round,
    NoRepeat,
}

property_keywords_impl! { BackgroundRepeatKeyword,
    BackgroundRepeatKeyword::Repeat, "repeat",
    BackgroundRepeatKeyword::Space, "space",
    BackgroundRepeatKeyword::Round, "round",
    BackgroundRepeatKeyword::NoRepeat, "no-repeat",
}

pub type BackgroundRepeat = Pair<BackgroundRepeatKeyword>;

impl BackgroundRepeat {
    /// https://drafts.csswg.org/css-backgrounds/#background-repeat
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        input
            .try_parse(|input| {
                input.expect_ident_matching("repeat-x")?;
                Ok(BackgroundRepeat::new(
                    BackgroundRepeatKeyword::Repeat,
                    BackgroundRepeatKeyword::NoRepeat,
                ))
            })
            .or_else(|_err: ParseError<'i>| {
                input
                    .try_parse(|input| {
                        input.expect_ident_matching("repeat-y")?;
                        Ok(BackgroundRepeat::new(
                            BackgroundRepeatKeyword::NoRepeat,
                            BackgroundRepeatKeyword::Repeat,
                        ))
                    })
                    .or_else(|_err: ParseError<'i>| {
                        BackgroundRepeat::parse_with(input, |input| {
                            BackgroundRepeatKeyword::parse(input)
                        })
                    })
            })
    }
}

pub fn parse_declared<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
    BackgroundRepeat::parse(context, input).map(PropertyDeclaration::BackgroundRepeat)
}
