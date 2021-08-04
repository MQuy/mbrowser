use cssparser::Parser;

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;

#[derive(Clone)]
pub enum ListStyleType {
    Disc,
    None,
    Circle,
    Square,
    DisclosureOpen,
    DisclosureClosed,
    Decimal,
    LowerAlpha,
    UpperAlpha,
    ArabicIndic,
    Bengali,
    Cambodian,
    CjkDecimal,
    Devanagari,
    Gujarati,
    Gurmukhi,
    Kannada,
    Khmer,
    Lao,
    Malayalam,
    Mongolian,
    Myanmar,
    Oriya,
    Persian,
    Telugu,
    Thai,
    Tibetan,
    CjkEarthlyBranch,
    CjkHeavenlyStem,
    LowerGreek,
    Hiragana,
    HiraganaIroha,
    Katakana,
    KatakanaIroha,
}

pub fn parse<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<ListStyleType, ParseError<'i>> {
    todo!()
}

pub fn parse_declared<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
    parse(context, input).map(PropertyDeclaration::ListStyleType)
}
