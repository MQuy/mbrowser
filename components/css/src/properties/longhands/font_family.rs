use cssparser::{match_ignore_ascii_case, Parser, ToCss, Token, _cssparser_internal_to_lowercase};

use crate::parser::{parse_repeated, ParseError};
use crate::properties::declaration::{property_keywords_impl, PropertyDeclaration};
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::CustomIdent;

#[derive(Clone)]
pub enum GenericFamilyName {
    Serif,
    SansSerif,
    Cursive,
    Fantasy,
    Monospace,
    SystemUI,
    Emoji,
    Math,
    Fangsong,
    UISerif,
    UISansSerif,
    UIMonospace,
    UIRounded,
}

property_keywords_impl! { GenericFamilyName,
    GenericFamilyName::Serif, "serif",
    GenericFamilyName::SansSerif, "sans-serif",
    GenericFamilyName::Cursive, "cursive",
    GenericFamilyName::Fantasy, "fantasy",
    GenericFamilyName::Monospace, "monospace",
    GenericFamilyName::SystemUI, "system-ui",
    GenericFamilyName::Emoji, "emoji",
    GenericFamilyName::Math, "math",
    GenericFamilyName::Fangsong, "fangsong",
    GenericFamilyName::UISerif, "ui-serif",
    GenericFamilyName::UISansSerif, "ui-sans-serif",
    GenericFamilyName::UIMonospace, "ui-monospace",
    GenericFamilyName::UIRounded, "ui-rounded",
}

#[derive(Clone)]
pub enum FamilyName {
    String(String),
    Ident(Vec<CustomIdent>),
}

impl FamilyName {
    pub fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i>> {
        input
            .try_parse(|input| {
                let value = input.expect_string()?;
                Ok(FamilyName::String(value.to_string()))
            })
            .or_else(|_err: ParseError<'i>| {
                let mut idents = parse_repeated(input, &mut |input| CustomIdent::parse(input), 1)?;
                Ok(FamilyName::Ident(idents))
            })
    }
}

#[derive(Clone)]
pub enum SingleFontFamily {
    FamilyName(FamilyName),
    GenericFamily(GenericFamilyName),
}

impl SingleFontFamily {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        input
            .try_parse(|input| {
                let family = GenericFamilyName::parse(input)?;
                Ok(SingleFontFamily::GenericFamily(family))
            })
            .or_else(|_err: ParseError<'i>| {
                let name = FamilyName::parse(input)?;
                Ok(SingleFontFamily::FamilyName(name))
            })
    }
}

#[derive(Clone)]
pub struct FontFamily(Vec<SingleFontFamily>);

impl FontFamily {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        let fonts = input.parse_comma_separated(|input| SingleFontFamily::parse(context, input))?;
        Ok(FontFamily(fonts))
    }
}

pub fn parse_declared<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
    FontFamily::parse(context, input).map(PropertyDeclaration::FontFamily)
}
