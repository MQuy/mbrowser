use cssparser::Parser;

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::number::NumberOrPercentage;

#[derive(Clone)]
#[repr(C, u8)]
pub enum Scale {
    None,
    Scale(NumberOrPercentage, NumberOrPercentage, NumberOrPercentage),
}

impl Scale {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Scale, ParseError<'i>> {
        input
            .try_parse(|input| {
                input.expect_ident_matching("none")?;
                Ok(Scale::None)
            })
            .or_else(|_err: ParseError<'i>| {
                let x = NumberOrPercentage::parse(context, input)?;
                let y = input
                    .try_parse(|input| NumberOrPercentage::parse(context, input))
                    .map_or(x.clone(), |value| value);
                let z = input
                    .try_parse(|input| NumberOrPercentage::parse(context, input))
                    .map_or("1".into(), |value| value);
                Ok(Scale::Scale(x, y, z))
            })
    }
}

pub fn parse_declared<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
    Scale::parse(context, input).map(PropertyDeclaration::Scale)
}
