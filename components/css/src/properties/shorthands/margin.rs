use cssparser::Parser;

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::properties::declaration_block::SourcePropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::length::LengthPercentageOrAuto;

pub struct Longhands {
    pub margin_top: LengthPercentageOrAuto,
    pub margin_right: LengthPercentageOrAuto,
    pub margin_bottom: LengthPercentageOrAuto,
    pub margin_left: LengthPercentageOrAuto,
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
            declarations.push(PropertyDeclaration::MarginTop(longhands.margin_top));
            declarations.push(PropertyDeclaration::MarginRight(longhands.margin_right));
            declarations.push(PropertyDeclaration::MarginBottom(longhands.margin_bottom));
            declarations.push(PropertyDeclaration::MarginLeft(longhands.margin_left));
        })
}
