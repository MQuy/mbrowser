use cssparser::Parser;

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::layout::LineStyle;

/// https://drafts.csswg.org/css-backgrounds-3/#propdef-border-right-style
pub fn parse_declared<'i, 't>(
	_context: &ParserContext,
	input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
	LineStyle::parse(input).map(PropertyDeclaration::BorderRightStyle)
}
