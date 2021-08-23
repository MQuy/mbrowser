use cssparser::{match_ignore_ascii_case, Parser, Token, _cssparser_internal_to_lowercase};

use crate::parser::ParseError;
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;

#[derive(Clone)]
pub enum LeaderType {
    Dotted,
    Solid,
    Space,
    String(String),
}

impl LeaderType {
    pub fn parse<'i, 't>(
        _context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        let location = input.current_source_location();
        let token = input.next()?;
        Ok(match token {
            Token::Ident(ident) => match_ignore_ascii_case! { ident,
                "dotted" => LeaderType::Dotted,
                "solid" => LeaderType::Solid,
                "space" => LeaderType::Space,
                _ => return Err(location.new_custom_error(StyleParseErrorKind::UnexpectedValue(ident.clone())))
            },
            Token::QuotedString(text) => LeaderType::String(text.to_string()),
            _ => {
                return Err(
                    location.new_custom_error(StyleParseErrorKind::UnexpectedToken(token.clone()))
                )
            },
        })
    }
}

#[derive(Clone)]
pub struct Leader(LeaderType);

impl Leader {
    /// https://drafts.csswg.org/css-content/#leader-function
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        input.expect_function_matching("leader")?;
        input.parse_nested_block(|input| {
            let style = LeaderType::parse(context, input)?;
            Ok(Leader(style))
        })
    }
}
