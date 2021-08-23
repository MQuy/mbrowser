use cssparser::{match_ignore_ascii_case, Parser, Token, _cssparser_internal_to_lowercase};

use super::CSSFloat;
use crate::parser::ParseError;
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;

#[derive(Clone)]
pub enum TimeUnit {
    Second,
    Millisecond,
}

#[derive(Clone)]
pub struct Time {
    amount: CSSFloat,
    unit: TimeUnit,
}

impl Time {
    /// https://drafts.csswg.org/css-values/#time
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        macro_rules! return_unexpected_token {
            ($location:tt, $token:tt) => {
                return Err($location
                    .new_custom_error(StyleParseErrorKind::UnexpectedToken($token.clone())))
            };
        }

        let location = input.current_source_location();
        let token = input.next()?;
        Ok(match token {
            Token::Dimension { unit, value, .. } => {
                let name = match_ignore_ascii_case! { unit,
                    "s" => TimeUnit::Second,
                    "ms" => TimeUnit::Millisecond,
                    _ => return_unexpected_token!(location, token),
                };
                Time {
                    amount: *value,
                    unit: name,
                }
            },
            _ => return_unexpected_token!(location, token),
        })
    }
}
