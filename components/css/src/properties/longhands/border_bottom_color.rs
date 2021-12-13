use cssparser::Parser;

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::specified::color::Color;

/// https://drafts.csswg.org/css-backgrounds/#propdef-border-bottom-color
pub fn parse_declared<'i, 't>(
	_context: &ParserContext,
	input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
	Color::parse(input).map(PropertyDeclaration::BorderBottomColor)
}
