use cssparser::Parser;

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::properties::declaration_block::SourcePropertyDeclaration;
use crate::properties::longhands::font_family::FontFamily;
use crate::properties::longhands::font_size::FontSize;
use crate::properties::longhands::font_stretch::FontStretch;
use crate::properties::longhands::font_style::FontStyle;
use crate::properties::longhands::font_variant_caps::FontVariantCaps;
use crate::properties::longhands::font_weight::FontWeight;
use crate::properties::longhands::line_height::LineHeight;
use crate::stylesheets::stylesheet::ParserContext;

pub struct Longhands {
    pub font_style: FontStyle,
    pub font_variant_caps: FontVariantCaps,
    pub font_weight: FontWeight,
    pub font_stretch: FontStretch,
    pub font_size: FontSize,
    pub line_height: LineHeight,
    pub font_family: FontFamily,
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
            declarations.push(PropertyDeclaration::FontStyle(longhands.font_style));
            declarations.push(PropertyDeclaration::FontVariantCaps(
                longhands.font_variant_caps,
            ));
            declarations.push(PropertyDeclaration::FontWeight(longhands.font_weight));
            declarations.push(PropertyDeclaration::FontStretch(longhands.font_stretch));
            declarations.push(PropertyDeclaration::FontSize(longhands.font_size));
            declarations.push(PropertyDeclaration::LineHeight(longhands.line_height));
            declarations.push(PropertyDeclaration::FontFamily(longhands.font_family));
        })
}
