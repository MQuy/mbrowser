use cssparser::{match_ignore_ascii_case, Parser, ToCss, Token, _cssparser_internal_to_lowercase};

use crate::parser::ParseError;
use crate::properties::declaration::{property_keywords_impl, PropertyDeclaration};
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;

/// https://drafts.csswg.org/css-fonts/#font-variant-caps-prop
#[derive(Clone, Debug)]
pub enum FontVariantCaps {
	Normal,
	SmallCaps,
	AllSmallCaps,
	PetiteCaps,
	AllPetiteCaps,
	Unicase,
	TitlingCaps,
}

property_keywords_impl! { FontVariantCaps,
	FontVariantCaps::Normal, "normal",
	FontVariantCaps::SmallCaps, "small-caps",
	FontVariantCaps::AllSmallCaps, "all-small-caps",
	FontVariantCaps::PetiteCaps, "petite-caps",
	FontVariantCaps::AllPetiteCaps, "all-petite-caps",
	FontVariantCaps::Unicase, "unicase",
	FontVariantCaps::TitlingCaps, "titling-caps",
}

pub fn parse_declared<'i, 't>(
	_context: &ParserContext,
	input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
	FontVariantCaps::parse(input).map(PropertyDeclaration::FontVariantCaps)
}
