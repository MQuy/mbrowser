use cssparser::{match_ignore_ascii_case, Parser, ToCss, Token, _cssparser_internal_to_lowercase};

use crate::parser::ParseError;
use crate::properties::declaration::{property_keywords_impl, PropertyDeclaration};
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;

/// https://drafts.fxtf.org/compositing/#mix-blend-mode
#[derive(Clone, Debug)]
pub enum MixBlendMode {
	Normal,
	Multiply,
	Screen,
	Overlay,
	Darken,
	Lighten,
	ColorDodge,
	ColorBurn,
	HardLight,
	SoftLight,
	Difference,
	Exclusion,
	Hue,
	Saturation,
	Color,
	Luminosity,
}

property_keywords_impl! { MixBlendMode,
	MixBlendMode::Normal, "normal",
	MixBlendMode::Multiply, "multiply",
	MixBlendMode::Screen, "screen",
	MixBlendMode::Overlay, "overlay",
	MixBlendMode::Darken, "darken",
	MixBlendMode::Lighten, "lighten",
	MixBlendMode::ColorDodge, "color-dodge",
	MixBlendMode::ColorBurn, "color-burn",
	MixBlendMode::HardLight, "hard-light",
	MixBlendMode::SoftLight, "soft-light",
	MixBlendMode::Difference, "difference",
	MixBlendMode::Exclusion, "exclusion",
	MixBlendMode::Hue, "hue",
	MixBlendMode::Saturation, "saturation",
	MixBlendMode::Color, "color",
	MixBlendMode::Luminosity, "luminosity",
}

pub fn parse_declared<'i, 't>(
	_context: &ParserContext,
	input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
	MixBlendMode::parse(input).map(PropertyDeclaration::MixBlendMode)
}
