use common::url::BrowserUrl;
use cssparser::{match_ignore_ascii_case, Parser, _cssparser_internal_to_lowercase};

use crate::parser::ParseError;
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;

#[derive(Clone, Debug)]
pub struct CssUrl {
    original: Option<String>,
    resolved: Option<BrowserUrl>,
}

impl CssUrl {
    pub fn parse<'i, 't>(
        _context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        let location = input.current_source_location();
        let name = input.expect_function()?.clone();
        let value = input.parse_nested_block(|input| {
            match_ignore_ascii_case! { &name,
                "url" | "src" => Ok(input.expect_string()?.to_string()),
                _ => Err(location.new_custom_error(StyleParseErrorKind::UnexpectedValue(name.clone())))
        }
        })?;
        let url = BrowserUrl::parse(&value)
            .map_err(|_err| input.new_custom_error(StyleParseErrorKind::UnspecifiedError))?;
        Ok(CssUrl {
            original: None,
            resolved: Some(url),
        })
    }

    pub fn parse_string<'i, 't>(
        _context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        let value = input.expect_string()?.to_string();
        Ok(CssUrl {
            original: Some(value),
            resolved: None,
        })
    }
}
