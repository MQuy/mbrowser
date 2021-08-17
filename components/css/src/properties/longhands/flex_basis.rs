use cssparser::Parser;

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::length::Size;

#[derive(Clone)]
pub enum FlexBasis {
    Content,
    Width(Size),
}

impl FlexBasis {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<FlexBasis, ParseError<'i>> {
        input
            .try_parse(|input| {
                input.expect_ident_matching("content")?;
                Ok(FlexBasis::Content)
            })
            .or_else(|_err: ParseError<'i>| {
                let size = Size::parse(context, input)?;
                Ok(FlexBasis::Width(size))
            })
    }
}

pub fn parse_declared<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
    FlexBasis::parse(context, input).map(PropertyDeclaration::FlexBasis)
}
