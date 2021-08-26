use cssparser::Parser;

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::color::Color;

/// https://drafts.csswg.org/css-backgrounds-3/#propdef-border-top-color
pub fn parse_declared<'i, 't>(
	context: &ParserContext,
	input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
	Color::parse(context, input).map(PropertyDeclaration::BorderTopColor)
}
