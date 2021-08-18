use cssparser::{match_ignore_ascii_case, Parser, ToCss, Token, _cssparser_internal_to_lowercase};

use crate::parser::ParseError;
use crate::properties::declaration::{property_keywords_impl, PropertyDeclaration};
use crate::stylesheets::rule_parser::StyleParseErrorKind;
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

property_keywords_impl! { ListStyleType,
    ListStyleType::Disc, "disc",
    ListStyleType::None, "none",
    ListStyleType::Circle, "circle",
    ListStyleType::Square, "square",
    ListStyleType::DisclosureOpen, "disclosure-open",
    ListStyleType::DisclosureClosed, "disclosure-closed",
    ListStyleType::Decimal, "decimal",
    ListStyleType::LowerAlpha, "lower-alpha",
    ListStyleType::UpperAlpha, "upper-alpha",
    ListStyleType::ArabicIndic, "arabic-indic",
    ListStyleType::Bengali, "bengali",
    ListStyleType::Cambodian, "cambodian",
    ListStyleType::CjkDecimal, "cjk-decimal",
    ListStyleType::Devanagari, "devanagari",
    ListStyleType::Gujarati, "gujarati",
    ListStyleType::Gurmukhi, "gurmukhi",
    ListStyleType::Kannada, "kannada",
    ListStyleType::Khmer, "khmer",
    ListStyleType::Lao, "lao",
    ListStyleType::Malayalam, "malayalam",
    ListStyleType::Mongolian, "mongolian",
    ListStyleType::Myanmar, "myanmar",
    ListStyleType::Oriya, "oriya",
    ListStyleType::Persian, "persian",
    ListStyleType::Telugu, "telugu",
    ListStyleType::Thai, "thai",
    ListStyleType::Tibetan, "tibetan",
    ListStyleType::CjkEarthlyBranch, "cjk-earthly-branch",
    ListStyleType::CjkHeavenlyStem, "cjk-heavenly-stem",
    ListStyleType::LowerGreek, "lower-greek",
    ListStyleType::Hiragana, "hiragana",
    ListStyleType::HiraganaIroha, "hiragana-iroha",
    ListStyleType::Katakana, "katakana",
    ListStyleType::KatakanaIroha, "katakana-iroha",
}

pub fn parse_declared<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
    ListStyleType::parse(input).map(PropertyDeclaration::ListStyleType)
}
