use cssparser::Parser;

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::length::LengthOrAutoRectAuto;

/// https://drafts.fxtf.org/css-masking/#clip-property
pub type Clip = LengthOrAutoRectAuto;

pub fn parse_declared<'i, 't>(
	context: &ParserContext,
	input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
	Clip::parse(context, input).map(PropertyDeclaration::Clip)
}
