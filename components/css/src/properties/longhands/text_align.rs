use cssparser::{match_ignore_ascii_case, Parser, ToCss, Token, _cssparser_internal_to_lowercase};

use crate::parser::ParseError;
use crate::properties::declaration::{property_keywords_impl, PropertyDeclaration};
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;

#[derive(Clone)]
#[repr(u8)]
pub enum TextAlignKeyword {
    Start,
    Left,
    Right,
    Center,
    Justify,
    End,
}

property_keywords_impl! { TextAlignKeyword,
    TextAlignKeyword::Start, "start",
    TextAlignKeyword::Left, "left",
    TextAlignKeyword::Right, "right",
    TextAlignKeyword::Center, "center",
    TextAlignKeyword::Justify, "justify",
    TextAlignKeyword::End, "end",
}

#[derive(Clone)]
pub enum TextAlign {
    Keyword(TextAlignKeyword),
}

impl TextAlign {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<TextAlign, ParseError<'i>> {
        todo!()
    }
}

pub fn parse_declared<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
    TextAlign::parse(context, input).map(PropertyDeclaration::TextAlign)
}
