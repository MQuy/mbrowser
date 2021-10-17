use cssparser::{match_ignore_ascii_case, Parser, ToCss, Token, _cssparser_internal_to_lowercase};

use crate::computed_values::StyleContext;
use crate::parser::{parse_repeated, ParseError};
use crate::properties::declaration::{
	property_keywords_impl, PropertyDeclaration, WideKeywordDeclaration,
};
use crate::properties::longhand_id::LonghandId;
use crate::properties::property_id::CSSWideKeyword;
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::{computed, CustomIdent};

#[derive(Clone, Debug)]
pub enum GenericFamilyName {
	Serif,
	SansSerif,
	Cursive,
	Fantasy,
	Monospace,
	SystemUI,
	Emoji,
	Math,
	Fangsong,
	UISerif,
	UISansSerif,
	UIMonospace,
	UIRounded,
}

property_keywords_impl! { GenericFamilyName,
	GenericFamilyName::Serif, "serif",
	GenericFamilyName::SansSerif, "sans-serif",
	GenericFamilyName::Cursive, "cursive",
	GenericFamilyName::Fantasy, "fantasy",
	GenericFamilyName::Monospace, "monospace",
	GenericFamilyName::SystemUI, "system-ui",
	GenericFamilyName::Emoji, "emoji",
	GenericFamilyName::Math, "math",
	GenericFamilyName::Fangsong, "fangsong",
	GenericFamilyName::UISerif, "ui-serif",
	GenericFamilyName::UISansSerif, "ui-sans-serif",
	GenericFamilyName::UIMonospace, "ui-monospace",
	GenericFamilyName::UIRounded, "ui-rounded",
}

#[derive(Clone, Debug)]
pub enum FamilyName {
	String(String),
	Ident(Vec<CustomIdent>),
}

impl FamilyName {
	pub fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i>> {
		input
			.try_parse(|input| {
				let value = input.expect_string()?;
				Ok(FamilyName::String(value.to_string()))
			})
			.or_else(|_err: ParseError<'i>| {
				let idents = parse_repeated(input, &mut |input| CustomIdent::parse(input), 1)?;
				Ok(FamilyName::Ident(idents))
			})
	}
}

impl ToCss for FamilyName {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		match self {
			FamilyName::String(value) => dest.write_fmt(std::format_args!("\"{}\"", value)),
			FamilyName::Ident(idents) => idents.iter().map(|v| v.to_css(dest)).collect(),
		}
	}
}

#[derive(Clone, Debug)]
pub enum SingleFontFamily {
	FamilyName(FamilyName),
	GenericFamily(GenericFamilyName),
}

impl SingleFontFamily {
	pub fn parse<'i, 't>(
		_context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		input
			.try_parse(|input| {
				let family = GenericFamilyName::parse(input)?;
				Ok(SingleFontFamily::GenericFamily(family))
			})
			.or_else(|_err: ParseError<'i>| {
				let name = FamilyName::parse(input)?;
				Ok(SingleFontFamily::FamilyName(name))
			})
	}
}

impl ToString for SingleFontFamily {
	fn to_string(&self) -> String {
		self.to_css_string()
	}
}

impl ToCss for SingleFontFamily {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		match self {
			SingleFontFamily::FamilyName(value) => value.to_css(dest),
			SingleFontFamily::GenericFamily(value) => value.to_css(dest),
		}
	}
}

/// https://drafts.csswg.org/css-fonts/#font-family-prop
#[derive(Clone, Debug)]
pub struct FontFamily(Vec<SingleFontFamily>);

impl FontFamily {
	pub fn parse<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		let fonts = input.parse_comma_separated(|input| SingleFontFamily::parse(context, input))?;
		Ok(FontFamily(fonts))
	}
}

impl ToCss for FontFamily {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		let values: Vec<String> = self.0.iter().map(|v| v.to_css_string()).collect();
		dest.write_str(&values.join(", "))
	}
}

pub fn initial_value() -> String {
	"sans-seri".to_string()
}

pub fn cascade_property<'a>(
	declaration: Option<&PropertyDeclaration>,
	context: &'a mut StyleContext,
) {
	let computed_value = computed::from_inherited_property!(
		declaration,
		context.parent_style.get_font_families().clone(),
		vec![initial_value()],
		LonghandId::FontFamily,
		PropertyDeclaration::FontFamily(value) => value.0.iter().map(|v| v.to_css_string()).collect()
	);
	context.computed_values.set_font_families(computed_value);
}

pub fn parse_declared<'i, 't>(
	context: &ParserContext,
	input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
	FontFamily::parse(context, input).map(PropertyDeclaration::FontFamily)
}
