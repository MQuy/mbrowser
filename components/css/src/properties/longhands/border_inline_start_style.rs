use cssparser::Parser;

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::specified::layout::LineStyle;

/// https://drafts.csswg.org/css-logical/#propdef-border-inline-start-color
pub fn parse_declared<'i, 't>(
	_context: &ParserContext,
	input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
	LineStyle::parse(input).map(PropertyDeclaration::BorderInlineStartStyle)
}
