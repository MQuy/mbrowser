use cssparser::{Parser, ToCss, _cssparser_internal_to_lowercase, match_ignore_ascii_case};

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::specified::angle::Angle;

/// https://drafts.csswg.org/css-fonts/#font-style-prop
#[derive(Clone)]
pub enum FontStyle {
    Normal,
    Italic,
    Oblique(Angle),
}

impl FontStyle {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        input
            .try_parse(|input| {
                let location = input.current_source_location();
                let ident = input.expect_ident()?;
                Ok(match_ignore_ascii_case! { ident,
                    "normal" => FontStyle::Normal,
                    "italic" => FontStyle::Italic,
                    "oblique" => {
                        let angle = input
                            .try_parse(|input| Angle::parse(context, input))
                            .map_or("14deg".into(), |angle| angle);
                        FontStyle::Oblique(angle)
                    },
                    _ => return Err(location.new_custom_error(StyleParseErrorKind::UnexpectedValue(ident.clone())))
                })
            })
    }
}

impl ToCss for FontStyle {
    fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
    where
        W: std::fmt::Write,
    {
        match self {
            FontStyle::Normal => dest.write_str("normal"),
            FontStyle::Italic => dest.write_str("italic"),
            FontStyle::Oblique(value) => {
                dest.write_str("oblique ")?;
                value.to_css(dest)
            },
        }
    }
}

pub fn parse_declared<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
    FontStyle::parse(context, input).map(PropertyDeclaration::FontStyle)
}
