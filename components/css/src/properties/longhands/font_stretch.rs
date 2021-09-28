use cssparser::{match_ignore_ascii_case, Parser, ToCss, Token, _cssparser_internal_to_lowercase};

use crate::parser::ParseError;
use crate::properties::declaration::{property_keywords_impl, PropertyDeclaration};
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::specified::percentage::Percentage;

#[derive(Clone, Debug)]
pub enum FontStretchKeyword {
	Normal,
	Condensed,
	UltraCondensed,
	ExtraCondensed,
	SemiCondensed,
	SemiExpanded,
	Expanded,
	ExtraExpanded,
	UltraExpanded,
}

property_keywords_impl! { FontStretchKeyword,
	FontStretchKeyword::Normal, "normal",
	FontStretchKeyword::Condensed, "condensed",
	FontStretchKeyword::UltraCondensed, "ultra-condensed",
	FontStretchKeyword::ExtraCondensed, "extra-condensed",
	FontStretchKeyword::SemiCondensed, "semi-condensed",
	FontStretchKeyword::SemiExpanded, "semi-expanded",
	FontStretchKeyword::Expanded, "expanded",
	FontStretchKeyword::ExtraExpanded, "extra-expanded",
	FontStretchKeyword::UltraExpanded, "ultra-expanded",
}

/// https://drafts.csswg.org/css-fonts/#font-stretch-prop
#[derive(Clone, Debug)]
pub enum FontStretch {
	Stretch(Percentage),
	Keyword(FontStretchKeyword),
}

impl FontStretch {
	pub fn parse<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<FontStretch, ParseError<'i>> {
		input
			.try_parse(|input| {
				let keyword = FontStretchKeyword::parse(input)?;
				Ok(FontStretch::Keyword(keyword))
			})
			.or_else(|_err: ParseError<'i>| {
				let percentage = Percentage::parse(context, input)?;
				Ok(FontStretch::Stretch(percentage))
			})
	}
}

impl ToCss for FontStretch {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		match self {
			FontStretch::Stretch(value) => value.to_css(dest),
			FontStretch::Keyword(value) => value.to_css(dest),
		}
	}
}

pub fn parse_declared<'i, 't>(
	context: &ParserContext,
	input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
	FontStretch::parse(context, input).map(PropertyDeclaration::FontStretch)
}
