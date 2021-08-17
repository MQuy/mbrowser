use cssparser::Parser;

use crate::parser::{parse_in_any_order, parse_item_if_missing, ParseError};
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::position::PreferredRatio;

#[derive(Clone)]
pub struct AspectRatio {
    pub auto: bool,
    pub ratio: PreferredRatio,
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
            context,
            input,
            &mut [
                &mut |context, input| {
                    parse_item_if_missing(context, input, &mut auto, |_context, input| {
                        input.expect_ident_matching("auto")?;
                        Ok(())
                    })
                },
                &mut |context, input| {
                    parse_item_if_missing(context, input, &mut ratio, |context, input| {
                        PreferredRatio::parse(context, input)
                    })
                },
            ],
        );
        if auto.is_none() && ratio.is_none() {
            Err(input.new_custom_error(StyleParseErrorKind::UnspecifiedError))
        } else {
            Ok(AspectRatio {
                auto: auto.is_some(),
                ratio: ratio.map_or(PreferredRatio::None, |ratio| ratio),
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
