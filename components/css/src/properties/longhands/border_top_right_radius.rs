use cssparser::Parser;

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::specified::border::BorderCornerRadius;

/// https://drafts.csswg.org/css-backgrounds-3/#propdef-border-top-right-radius
pub fn parse_declared<'i, 't>(
	_context: &ParserContext,
	input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
	BorderCornerRadius::parse(input).map(PropertyDeclaration::BorderTopRightRadius)
}
