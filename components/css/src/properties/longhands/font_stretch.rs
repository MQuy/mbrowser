use cssparser::Parser;

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::percentage::Percentage;

#[derive(Clone)]
pub enum FontStretchKeyword {
    Normal,
    Condensed,
    UltraCondensed,
    ExtraCondensed,
    SemiCondensed,
    SemiExpanded,
    Expanded,
    ExtraExpanded,
    UltraExpanded,
}

#[derive(Clone)]
#[repr(u8)]
pub enum FontStretch {
    Stretch(Percentage),
    Keyword(FontStretchKeyword),
}

pub fn parse<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<FontStretch, ParseError<'i>> {
    todo!()
}

pub fn parse_declared<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
    parse(context, input).map(PropertyDeclaration::FontStretch)
}
