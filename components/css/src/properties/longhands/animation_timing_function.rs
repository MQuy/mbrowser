use cssparser::{Parser, ToCss};

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::specified::easing::EasingFunction;

/// https://drafts.csswg.org/css-animations-1/#animation-timing-function
#[derive(Clone)]
pub struct AnimationTimingFunction {
	timing: Vec<EasingFunction>,
}

impl AnimationTimingFunction {
	pub fn parse<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		let timing = input.parse_comma_separated(|input| EasingFunction::parse(context, input))?;
		Ok(AnimationTimingFunction { timing })
	}
}

impl ToCss for AnimationTimingFunction {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		let timing: Vec<String> = self.timing.iter().map(|t| t.to_css_string()).collect();
		dest.write_str(&timing.join(", "))
	}
}

pub fn parse_declared<'i, 't>(
	context: &ParserContext,
	input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
	AnimationTimingFunction::parse(context, input).map(PropertyDeclaration::AnimationTimingFunction)
}
