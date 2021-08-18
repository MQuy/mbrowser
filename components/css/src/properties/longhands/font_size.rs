use cssparser::{match_ignore_ascii_case, Parser, ToCss, Token, _cssparser_internal_to_lowercase};

use crate::parser::ParseError;
use crate::properties::declaration::{property_keywords_impl, PropertyDeclaration};
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::length::LengthPercentage;

#[derive(Clone)]
pub enum AbsoluteSize {
    XXSmall,
    XSmall,
    Small,
    Medium,
    Large,
    XLarge,
    XXLarge,
    XXXLarge,
}

property_keywords_impl! { AbsoluteSize,
    AbsoluteSize::XXSmall, "xx-small",
    AbsoluteSize::XSmall, "x-small",
    AbsoluteSize::Small, "small",
    AbsoluteSize::Medium, "medium",
    AbsoluteSize::Large, "large",
    AbsoluteSize::XLarge, "x-large",
    AbsoluteSize::XXLarge, "xx-large",
    AbsoluteSize::XXXLarge, "xxx-large",
}

#[derive(Clone)]
pub enum RelativeSize {
    Larger,
    Smaller,
}

property_keywords_impl! { RelativeSize,
    RelativeSize::Larger, "larger",
    RelativeSize::Smaller, "smaller",
}

#[derive(Clone)]
pub enum FontSize {
    AbsoluteSize(AbsoluteSize),
    RelativeSize(RelativeSize),
    LengthPercentage(LengthPercentage),
}

impl FontSize {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<FontSize, ParseError<'i>> {
        input
            .try_parse(|input| {
                let size = AbsoluteSize::parse(input)?;
                Ok(FontSize::AbsoluteSize(size))
            })
            .or_else(|_err: ParseError<'i>| {
                let size = RelativeSize::parse(input)?;
                Ok(FontSize::RelativeSize(size))
            })
            .or_else(|_err: ParseError<'i>| {
                let value = LengthPercentage::parse(context, input)?;
                Ok(FontSize::LengthPercentage(value))
            })
    }
}

pub fn parse_declared<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
    FontSize::parse(context, input).map(PropertyDeclaration::FontSize)
}
