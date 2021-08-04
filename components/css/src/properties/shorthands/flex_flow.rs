use cssparser::Parser;

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::properties::declaration_block::SourcePropertyDeclaration;
use crate::properties::longhands::flex_direction::FlexDirection;
use crate::properties::longhands::flex_wrap::FlexWrap;
use crate::stylesheets::stylesheet::ParserContext;

pub struct Longhands {
    pub flex_direction: FlexDirection,
    pub flex_wrap: FlexWrap,
}

pub fn parse_value<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<Longhands, ParseError<'i>> {
    todo!()
}

/// Parse the given shorthand and fill the result into the
/// `declarations` vector.
pub fn parse_into<'i, 't>(
    declarations: &mut SourcePropertyDeclaration,
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<(), ParseError<'i>> {
    input
        .parse_entirely(|input| parse_value(context, input))
        .map(|longhands| {
            declarations.push(PropertyDeclaration::FlexDirection(longhands.flex_direction));
            declarations.push(PropertyDeclaration::FlexWrap(longhands.flex_wrap));
        })
}
