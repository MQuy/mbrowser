use cssparser::Parser;

use crate::parser::{parse_repeated, ParseError};
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::specified::transform::TransformFunction;

#[derive(Clone)]
pub struct Transform(Vec<TransformFunction>);

impl Transform {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Transform, ParseError<'i>> {
        input
            .try_parse(|input| {
                input.expect_ident_matching("none")?;
                Ok(Transform(vec![]))
            })
            .or_else(|_err: ParseError<'i>| {
                let transforms = parse_repeated(
                    input,
                    &mut |input| TransformFunction::parse(context, input),
                    1,
                )?;
                Ok(Transform(transforms))
            })
    }
}

pub fn parse_declared<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
    Transform::parse(context, input).map(PropertyDeclaration::Transform)
}
