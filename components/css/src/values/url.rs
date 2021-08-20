use common::url::BrowserUrl;
use cssparser::Parser;

use crate::parser::ParseError;
use crate::stylesheets::stylesheet::ParserContext;

#[derive(Clone, Debug)]
pub struct CssUrl {
    original: Option<String>,
    resolved: Option<BrowserUrl>,
}

impl CssUrl {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        todo!()
    }

    pub fn parse_string<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        todo!()
    }
}
