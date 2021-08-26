use cssparser::{Parser, ToCss};

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::time::Time;

/// https://drafts.csswg.org/css-transitions/#transition-duration-property
#[derive(Clone)]
pub struct TransitionDuration {
	durations: Vec<Time>,
}

impl TransitionDuration {
	pub fn parse<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		let delays = input.parse_comma_separated(|input| Time::parse(context, input))?;
		Ok(TransitionDuration { durations: delays })
	}
}

impl ToCss for TransitionDuration {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		let values: Vec<String> = self.durations.iter().map(|v| v.to_css_string()).collect();
		dest.write_str(&values.join(", "))
	}
}

pub fn parse_declared<'i, 't>(
	context: &ParserContext,
	input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
	TransitionDuration::parse(context, input).map(PropertyDeclaration::TransitionDuration)
}
