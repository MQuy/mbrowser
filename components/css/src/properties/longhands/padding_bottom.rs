use cssparser::Parser;

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::length::NonNegativeLengthPercentage;

/// https://drafts.csswg.org/css-box-4/#propdef-padding-bottom
pub fn parse_declared<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
    NonNegativeLengthPercentage::parse(context, input).map(PropertyDeclaration::PaddingBottom)
}
