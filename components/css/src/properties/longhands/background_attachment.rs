use cssparser::Parser;

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;

#[derive(Clone)]
pub enum Attachment {
    Scroll,
    Fixed,
    Local,
}

#[derive(Clone)]
pub struct BackgroundAttachment {
    attachments: Vec<Attachment>,
}

pub fn parse<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<BackgroundAttachment, ParseError<'i>> {
    panic!()
}

pub fn parse_declared<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
    parse(context, input).map(PropertyDeclaration::BackgroundAttachment)
}
