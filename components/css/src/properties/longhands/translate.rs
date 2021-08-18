use cssparser::Parser;

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::length::{Length, LengthPercentage};

#[derive(Clone)]
pub enum Translate {
    None,
    LengthPercentage(LengthPercentage, LengthPercentage, Length),
}

impl Translate {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        input
            .try_parse(|input| {
                input.expect_ident_matching("none")?;
                Ok(Translate::None)
            })
            .or_else(|_err: ParseError<'i>| {
                let x = input.try_parse(|input| LengthPercentage::parse(context, input))?;
                let y = if let Ok(y) =
                    input.try_parse(|input| LengthPercentage::parse(context, input))
                {
                    y
                } else {
                    return Ok(Translate::LengthPercentage(x, "0px".into(), "0px".into()));
                };
                let z = if let Ok(z) = input.try_parse(|input| Length::parse(context, input)) {
                    z
                } else {
                    return Ok(Translate::LengthPercentage(x, y, "0px".into()));
                };
                Ok(Translate::LengthPercentage(x, y, z))
            })
    }
}

pub fn parse_declared<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
    Translate::parse(context, input).map(PropertyDeclaration::Translate)
}
