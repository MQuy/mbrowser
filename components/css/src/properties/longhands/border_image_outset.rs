use cssparser::Parser;

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::length::NonNegativeLengthOrNumberRect;

pub type BorderImageOutset = Box<NonNegativeLengthOrNumberRect>;

/// https://drafts.csswg.org/css-backgrounds/#the-border-image-outset
pub fn parse<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<BorderImageOutset, ParseError<'i>> {
    let rect = NonNegativeLengthOrNumberRect::parse(context, input)?;
    Ok(Box::new(rect))
}

pub fn parse_declared<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
    parse(context, input).map(PropertyDeclaration::BorderImageOutset)
}
