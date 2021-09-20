use cssparser::{Parser, ToCss};

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::layout::Box;

/// https://drafts.csswg.org/css-backgrounds/#background-origin
#[derive(Clone, Debug)]
pub struct BackgroundOrigin {
	boxes: Vec<Box>,
}

impl BackgroundOrigin {
	pub fn parse<'i, 't>(
		_context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		let boxes = input.parse_comma_separated(Box::parse)?;
		Ok(BackgroundOrigin { boxes })
	}
}

impl ToCss for BackgroundOrigin {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		let values: Vec<String> = self.boxes.iter().map(|v| v.to_css_string()).collect();
		dest.write_str(&values.join(", "))
	}
}

pub fn parse_declared<'i, 't>(
	context: &ParserContext,
	input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
	BackgroundOrigin::parse(context, input).map(PropertyDeclaration::BackgroundOrigin)
}
