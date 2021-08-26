use cssparser::{match_ignore_ascii_case, Parser, ToCss, Token, _cssparser_internal_to_lowercase};

use crate::parser::ParseError;
use crate::properties::declaration::{property_keywords_impl, PropertyDeclaration};
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;

/// https://svgwg.org/svg2-draft/interact.html#PointerEventsProp
#[derive(Clone)]
pub enum PointerEvents {
	Auto,
	BoundingBox,
	VisiblePainted,
	VisibleFill,
	VisibleStroke,
	Visible,
	Painted,
	Fill,
	Stroke,
	All,
	None,
}

property_keywords_impl! { PointerEvents,
	PointerEvents::Auto, "auto",
	PointerEvents::BoundingBox, "bounding-box",
	PointerEvents::VisiblePainted, "visiblepainted",
	PointerEvents::VisibleFill, "visiblefill",
	PointerEvents::VisibleStroke, "visiblestroke",
	PointerEvents::Visible, "visible",
	PointerEvents::Painted, "painted",
	PointerEvents::Fill, "fill",
	PointerEvents::Stroke, "stroke",
	PointerEvents::All, "all",
	PointerEvents::None, "none",
}

pub fn parse_declared<'i, 't>(
	context: &ParserContext,
	input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
	PointerEvents::parse(input).map(PropertyDeclaration::PointerEvents)
}
