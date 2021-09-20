use cssparser::{match_ignore_ascii_case, Parser, ToCss, Token, _cssparser_internal_to_lowercase};

use crate::parser::ParseError;
use crate::properties::declaration::{property_keywords_impl, PropertyDeclaration};
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;

#[derive(Clone, Debug)]
pub enum SingleAnimationPlayState {
	Running,
	Paused,
}

property_keywords_impl! { SingleAnimationPlayState,
	SingleAnimationPlayState::Running, "running",
	SingleAnimationPlayState::Paused, "paused",
}

/// https://drafts.csswg.org/css-animations-1/#animation-play-state
#[derive(Clone, Debug)]
pub struct AnimationPlayState {
	play_states: Vec<SingleAnimationPlayState>,
}

impl AnimationPlayState {
	pub fn parse<'i, 't>(
		_context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		let play_states = input.parse_comma_separated(SingleAnimationPlayState::parse)?;
		Ok(AnimationPlayState { play_states })
	}
}

impl ToCss for AnimationPlayState {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		let states: Vec<String> = self.play_states.iter().map(|v| v.to_css_string()).collect();
		dest.write_str(&states.join(", "))
	}
}

pub fn parse_declared<'i, 't>(
	context: &ParserContext,
	input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
	AnimationPlayState::parse(context, input).map(PropertyDeclaration::AnimationPlayState)
}
