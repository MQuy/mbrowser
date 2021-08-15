use cssparser::Parser;

use crate::parser::ParseError;
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
        let mut auto = false;
        let mut ratio: PreferredRatio = PreferredRatio::None;
        loop {
            let auto_parser_ret = input.try_parse(|input| -> Result<(), ParseError<'i>> {
                input.expect_ident_matching("auto")?;
                auto = true;
                Ok(())
            });
            let ratio_parser_ret = input.try_parse(|input| -> Result<(), ParseError<'i>> {
                ratio = PreferredRatio::parse(context, input)?;
                Ok(())
            });

            if auto_parser_ret.is_err() && ratio_parser_ret.is_err() {
                break;
            }
        }
        if auto == false && ratio == PreferredRatio::None {
            Err(input.new_custom_error(StyleParseErrorKind::UnspecifiedError))
        } else {
            Ok(AspectRatio { auto, ratio })
        }
    }
}

pub fn parse_declared<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
    AspectRatio::parse(context, input).map(PropertyDeclaration::AspectRatio)
}
