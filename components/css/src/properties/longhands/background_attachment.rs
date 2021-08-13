use cssparser::{match_ignore_ascii_case, Parser, Token, _cssparser_internal_to_lowercase};

use crate::parser::ParseError;
use crate::properties::declaration::{property_keywords_impl, PropertyDeclaration};
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;

#[derive(Clone)]
pub enum Attachment {
    Scroll,
    Fixed,
    Local,
}

property_keywords_impl! { Attachment,
    Attachment::Scroll, "scroll",
    Attachment::Fixed, "fixed",
    Attachment::Local, "local",
}

#[derive(Clone)]
pub struct BackgroundAttachment {
    attachments: Vec<Attachment>,
}

impl BackgroundAttachment {
    pub fn parse<'i, 't>(
        _context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        let attachments = input.parse_comma_separated(Attachment::parse)?;
        Ok(BackgroundAttachment { attachments })
    }
}

pub fn parse_declared<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
    BackgroundAttachment::parse(context, input).map(PropertyDeclaration::BackgroundAttachment)
}
