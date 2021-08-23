use cssparser::Parser;

use crate::parser::{parse_in_any_order, parse_item_if_missing, ParseError};
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::percentage::Ratio;

#[derive(Clone)]
pub struct AspectRatio {
    pub auto: bool,
    pub ratio: Option<Ratio>,
}

impl AspectRatio {
    /// https://drafts.csswg.org/css-sizing-4/#aspect-ratio
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

pub fn parse_declared<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
    AspectRatio::parse(context, input).map(PropertyDeclaration::AspectRatio)
}
