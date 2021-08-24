use cssparser::Parser;

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::number::NumberOrPercentage;

/// https://drafts.csswg.org/css-color/#transparency
pub type Opacity = NumberOrPercentage;

pub fn parse_declared<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
    Opacity::parse(context, input).map(PropertyDeclaration::Opacity)
}
