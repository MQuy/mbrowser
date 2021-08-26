use cssparser::{match_ignore_ascii_case, Parser, ToCss, Token, _cssparser_internal_to_lowercase};

use crate::parser::ParseError;
use crate::properties::declaration::{property_keywords_impl, PropertyDeclaration};
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;

#[derive(Clone)]
pub enum SingleAnimationFillMode {
	None,
	Forwards,
	Backwards,
	Both,
}

property_keywords_impl! { SingleAnimationFillMode,
	SingleAnimationFillMode::None, "none",
	SingleAnimationFillMode::Forwards, "forwards",
	SingleAnimationFillMode::Backwards, "backwards",
	SingleAnimationFillMode::Both, "both",
}

/// https://drafts.csswg.org/css-animations-1/#animation-fill-mode
#[derive(Clone)]
pub struct AnimationFillMode {
	fill_modes: Vec<SingleAnimationFillMode>,
}

impl AnimationFillMode {
	pub fn parse<'i, 't>(
		_context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		let fill_modes = input.parse_comma_separated(SingleAnimationFillMode::parse)?;
		Ok(AnimationFillMode { fill_modes })
	}
}

impl ToCss for AnimationFillMode {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		let modes: Vec<String> = self.fill_modes.iter().map(|v| v.to_css_string()).collect();
		dest.write_str(&modes.join(", "))
	}
}

pub fn parse_declared<'i, 't>(
	context: &ParserContext,
	input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
	AnimationFillMode::parse(context, input).map(PropertyDeclaration::AnimationFillMode)
}
