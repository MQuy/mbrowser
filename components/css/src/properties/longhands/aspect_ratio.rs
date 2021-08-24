use cssparser::{Parser, ToCss};

use crate::css_writer::write_elements;
use crate::parser::{parse_in_any_order, parse_item_if_missing, ParseError};
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::percentage::Ratio;

/// https://drafts.csswg.org/css-sizing-4/#aspect-ratio
#[derive(Clone)]
pub struct AspectRatio {
    pub auto: bool,
    pub ratio: Option<Ratio>,
}

impl AspectRatio {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<AspectRatio, ParseError<'i>> {
        let mut auto = None;
        let mut ratio = None;
        parse_in_any_order(
            input,
            &mut [
                &mut |input| {
                    parse_item_if_missing(input, &mut auto, |_, input| {
                        input.expect_ident_matching("auto")?;
                        Ok(())
                    })
                },
                &mut |input| {
                    parse_item_if_missing(input, &mut ratio, |_, input| {
                        Ratio::parse(context, input)
                    })
                },
            ],
        );
        if auto.is_none() && ratio.is_none() {
            Err(input.new_custom_error(StyleParseErrorKind::UnspecifiedError))
        } else {
            Ok(AspectRatio {
                auto: auto.is_some(),
                ratio: ratio.map_or(None, |ratio| Some(ratio)),
            })
        }
    }
}

impl ToCss for AspectRatio {
    fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
    where
        W: std::fmt::Write,
    {
        let auto = if self.auto { Some("auto") } else { None };
        let ratio = self.ratio.as_ref().map(|ratio| ratio.to_css_string());
        write_elements(dest, &[auto, ratio.as_deref()], ' ')
    }
}

pub fn parse_declared<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
    AspectRatio::parse(context, input).map(PropertyDeclaration::AspectRatio)
}
