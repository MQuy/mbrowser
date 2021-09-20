use cssparser::{match_ignore_ascii_case, Parser, ToCss, Token, _cssparser_internal_to_lowercase};

use crate::parser::ParseError;
use crate::properties::declaration::{property_keywords_impl, PropertyDeclaration};
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;

#[derive(Clone, Debug)]
pub enum SingleAnimationDirection {
	Normal,
	Reverse,
	Alternate,
	AlternateReverse,
}

property_keywords_impl! { SingleAnimationDirection,
	SingleAnimationDirection::Normal, "normal",
	SingleAnimationDirection::Reverse, "reverse",
	SingleAnimationDirection::Alternate, "alternate",
	SingleAnimationDirection::AlternateReverse, "alternate-reverse",
}

/// https://drafts.csswg.org/css-animations-1/#animation-direction
#[derive(Clone, Debug)]
pub struct AnimationDirection {
	directions: Vec<SingleAnimationDirection>,
}

impl AnimationDirection {
	pub fn parse<'i, 't>(
		_context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		let directions = input.parse_comma_separated(SingleAnimationDirection::parse)?;
		Ok(AnimationDirection { directions })
	}
}

impl ToCss for AnimationDirection {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		let directions: Vec<String> = self.directions.iter().map(|v| v.to_css_string()).collect();
		dest.write_str(&directions.join(", "))
	}
}
pub fn parse_declared<'i, 't>(
	context: &ParserContext,
	input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
	AnimationDirection::parse(context, input).map(PropertyDeclaration::AnimationDirection)
}
