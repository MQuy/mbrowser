use cssparser::Parser;

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::properties::declaration_block::SourcePropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::border::BorderCornerRadius;

pub struct Longhands {
    pub border_top_left_radius: BorderCornerRadius,
    pub border_top_right_radius: BorderCornerRadius,
    pub border_bottom_right_radius: BorderCornerRadius,
    pub border_bottom_left_radius: BorderCornerRadius,
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
            declarations.push(PropertyDeclaration::BorderTopLeftRadius(
                longhands.border_top_left_radius,
            ));
            declarations.push(PropertyDeclaration::BorderTopRightRadius(
                longhands.border_top_right_radius,
            ));
            declarations.push(PropertyDeclaration::BorderBottomRightRadius(
                longhands.border_bottom_right_radius,
            ));
            declarations.push(PropertyDeclaration::BorderBottomLeftRadius(
                longhands.border_bottom_left_radius,
            ));
        })
}
