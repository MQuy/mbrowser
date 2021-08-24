use cssparser::Parser;

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::properties::declaration_block::SourcePropertyDeclaration;
use crate::properties::longhands::outline_color::OutlineColor;
use crate::properties::longhands::outline_style::OutlineStyle;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::specified::line::LineWidth;

pub struct Longhands {
    pub outline_color: OutlineColor,
    pub outline_style: OutlineStyle,
    pub outline_width: LineWidth,
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
            declarations.push(PropertyDeclaration::OutlineColor(longhands.outline_color));
            declarations.push(PropertyDeclaration::OutlineStyle(longhands.outline_style));
            declarations.push(PropertyDeclaration::OutlineWidth(longhands.outline_width));
        })
}
