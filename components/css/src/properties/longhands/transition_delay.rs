use cssparser::{Parser, ToCss};

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::time::Time;

/// https://drafts.csswg.org/css-transitions/#transition-delay-property
#[derive(Clone, Debug)]
pub struct TransitionDelay {
	delays: Vec<Time>,
}

impl TransitionDelay {
	pub fn parse<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		let delays = input.parse_comma_separated(|input| Time::parse(context, input))?;
		Ok(TransitionDelay { delays })
	}
}

impl ToCss for TransitionDelay {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		let values: Vec<String> = self.delays.iter().map(|v| v.to_css_string()).collect();
		dest.write_str(&values.join(", "))
	}
}

pub fn parse_declared<'i, 't>(
	context: &ParserContext,
	input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
	TransitionDelay::parse(context, input).map(PropertyDeclaration::TransitionDelay)
}
