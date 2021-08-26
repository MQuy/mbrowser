use cssparser::Parser;

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::border::BorderCornerRadius;

/// https://drafts.csswg.org/css-logical/#propdef-border-end-end-radius
pub fn parse_declared<'i, 't>(
	context: &ParserContext,
	input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
	BorderCornerRadius::parse(context, input).map(PropertyDeclaration::BorderEndEndRadius)
}
