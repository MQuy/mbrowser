use cssparser::Parser;

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::length::NonNegativeLengthOrNumberRect;

/// https://drafts.csswg.org/css-backgrounds/#the-border-image-outset
pub fn parse_declared<'i, 't>(
	context: &ParserContext,
	input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
	NonNegativeLengthOrNumberRect::parse(context, input).map(PropertyDeclaration::BorderImageOutset)
}
