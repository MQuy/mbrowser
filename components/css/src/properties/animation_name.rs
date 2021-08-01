use cssparser::{ParseError, Parser, Token};

use crate::parser::Parse;
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::CustomIdent;

#[derive(Clone)]
pub enum KeyframesName {
    None,
    Ident(CustomIdent),
    QuotedString(String),
}

#[derive(Clone)]
pub struct AnimationName {
    names: Vec<KeyframesName>,
}

impl Parse for KeyframesName {
    fn parse<'i, 't>(
        _context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i, StyleParseErrorKind<'i>>> {
        let location = input.current_source_location();
        match *input.next()? {
            Token::Ident(ref s) => Ok(KeyframesName::Ident(CustomIdent::from_ident(
                location,
                s,
                &["none"],
            )?)),
            Token::QuotedString(ref s) => Ok(KeyframesName::QuotedString(s.to_string())),
            ref t => Err(location.new_unexpected_token_error(t.clone())),
        }
    }
}
