use cssparser::{Parser, ToCss, _cssparser_internal_to_lowercase, match_ignore_ascii_case};

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::number::NonNegativeNumber;

/// https://drafts.csswg.org/css-fonts/#font-weight-prop
#[derive(Clone)]
pub enum FontWeight {
    Weight(NonNegativeNumber),
    Normal,
    Bold,
    Bolder,
    Lighter,
}

impl FontWeight {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        input.try_parse(|input| {
            let location = input.current_source_location();
            let ident = input.expect_ident()?;
            Ok(match_ignore_ascii_case! { ident,
                "bolder" => FontWeight::Bolder,
                "lighter" => FontWeight::Lighter,
                "normal" => FontWeight::Normal,
                "bold" => FontWeight::Bold,
                _ => {
                    return Err(
                        location.new_custom_error(StyleParseErrorKind::UnexpectedValue(ident.clone()))
                    )
                },
            })
            .or_else(|_err: ParseError<'i>| {
                let value = NonNegativeNumber::parse_in_range(
                    context,
                    input,
                    0.0,
                    1000.0,
                )?;
                Ok(FontWeight::Weight(value))
            })
        })
    }
}

impl ToCss for FontWeight {
    fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
    where
        W: std::fmt::Write,
    {
        match self {
            FontWeight::Weight(value) => value.to_css(dest),
            FontWeight::Normal => dest.write_str("normal"),
            FontWeight::Bold => dest.write_str("bold"),
            FontWeight::Bolder => dest.write_str("bolder"),
            FontWeight::Lighter => dest.write_str("lighter"),
        }
    }
}

pub fn parse_declared<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
    FontWeight::parse(context, input).map(PropertyDeclaration::FontWeight)
}
