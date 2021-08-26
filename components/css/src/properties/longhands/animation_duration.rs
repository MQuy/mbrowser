use cssparser::{Parser, ToCss};

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::time::Time;

/// https://drafts.csswg.org/css-animations-1/#animation-duration
#[derive(Clone)]
pub struct AnimationDuration {
	durations: Vec<Time>,
}

impl AnimationDuration {
	pub fn parse<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		let times = input.parse_comma_separated(|input| Time::parse(context, input))?;
		Ok(AnimationDuration { durations: times })
	}
}

impl ToCss for AnimationDuration {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		let durations: Vec<String> = self.durations.iter().map(|v| v.to_css_string()).collect();
		dest.write_str(&durations.join(", "))
	}
}

pub fn parse_declared<'i, 't>(
	context: &ParserContext,
	input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
	AnimationDuration::parse(context, input).map(PropertyDeclaration::AnimationDuration)
}
