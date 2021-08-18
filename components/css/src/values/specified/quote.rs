use cssparser::{match_ignore_ascii_case, Parser, ToCss, Token, _cssparser_internal_to_lowercase};

use crate::parser::ParseError;
use crate::properties::declaration::property_keywords_impl;
use crate::stylesheets::rule_parser::StyleParseErrorKind;

/// https://drafts.csswg.org/css-content/#typedef-quote
#[derive(Clone)]
pub enum Quote {
    OpenQuote,
    CloseQuote,
    NoOpenQuote,
    NoCloseQuote,
}

property_keywords_impl! { Quote,
    Quote::OpenQuote, "open-quote",
    Quote::CloseQuote, "close-quote",
    Quote::NoOpenQuote, "no-open-quote",
    Quote::NoCloseQuote, "no-close-quote",
}
