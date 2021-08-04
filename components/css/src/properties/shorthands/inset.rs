use cssparser::Parser;

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::properties::declaration_block::SourcePropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::length::LengthPercentageOrAuto;

pub struct Longhands {
    pub top: LengthPercentageOrAuto,
    pub right: LengthPercentageOrAuto,
    pub bottom: LengthPercentageOrAuto,
    pub left: LengthPercentageOrAuto,
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
            declarations.push(PropertyDeclaration::Top(longhands.top));
            declarations.push(PropertyDeclaration::Right(longhands.right));
            declarations.push(PropertyDeclaration::Bottom(longhands.bottom));
            declarations.push(PropertyDeclaration::Left(longhands.left));
        })
}
