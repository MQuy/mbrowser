use cssparser::{Parser, ToCss};

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::specified::easing::EasingFunction;

/// https://drafts.csswg.org/css-transitions/#transition-timing-function-property
#[derive(Clone, Debug)]
pub struct TransitionTimingFunction(Vec<EasingFunction>);

impl TransitionTimingFunction {
	pub fn parse<'i, 't>(context: &ParserContext, input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i>> {
		let values = input.parse_comma_separated(|input| EasingFunction::parse(context, input))?;
		Ok(TransitionTimingFunction(values))
	}
}

impl ToCss for TransitionTimingFunction {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		let values: Vec<String> = self.0.iter().map(|v| v.to_css_string()).collect();
		dest.write_str(&values.join(", "))
	}
}

pub fn parse_declared<'i, 't>(
	context: &ParserContext,
	input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
	TransitionTimingFunction::parse(context, input).map(PropertyDeclaration::TransitionTimingFunction)
}
