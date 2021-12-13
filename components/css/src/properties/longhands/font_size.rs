use common::not_supported;
use cssparser::{match_ignore_ascii_case, Parser, ToCss, Token, _cssparser_internal_to_lowercase};

use crate::computed_values::StyleContext;
use crate::parser::ParseError;
use crate::properties::declaration::{property_keywords_impl, PropertyDeclaration, WideKeywordDeclaration};
use crate::properties::longhand_id::LonghandId;
use crate::properties::property_id::CSSWideKeyword;
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::specified::length::LengthPercentage;
use crate::values::{computed, CSSFloat};

pub static DEFAULT_FONT_SIZE: f32 = 16.0;

#[derive(Clone, Debug)]
pub enum AbsoluteSize {
	XXSmall,
	XSmall,
	Small,
	Medium,
	Large,
	XLarge,
	XXLarge,
	XXXLarge,
}

impl AbsoluteSize {
	/// https://drafts.csswg.org/css-fonts/#absolute-size-mapping
	pub fn to_computed_value(&self) -> CSSFloat {
		match self {
			AbsoluteSize::XXSmall => 9.0,
			AbsoluteSize::XSmall => 12.0,
			AbsoluteSize::Small => 14.0,
			AbsoluteSize::Medium => 16.0,
			AbsoluteSize::Large => 19.0,
			AbsoluteSize::XLarge => 24.0,
			AbsoluteSize::XXLarge => 32.0,
			AbsoluteSize::XXXLarge => 48.0,
		}
	}
}

property_keywords_impl! { AbsoluteSize,
	AbsoluteSize::XXSmall, "xx-small",
	AbsoluteSize::XSmall, "x-small",
	AbsoluteSize::Small, "small",
	AbsoluteSize::Medium, "medium",
	AbsoluteSize::Large, "large",
	AbsoluteSize::XLarge, "x-large",
	AbsoluteSize::XXLarge, "xx-large",
	AbsoluteSize::XXXLarge, "xxx-large",
}

#[derive(Clone, Debug)]
pub enum RelativeSize {
	Larger,
	Smaller,
}

property_keywords_impl! { RelativeSize,
	RelativeSize::Larger, "larger",
	RelativeSize::Smaller, "smaller",
}

/// https://drafts.csswg.org/css-fonts/#font-size-prop
#[derive(Clone, Debug)]
pub enum FontSize {
	AbsoluteSize(AbsoluteSize),
	RelativeSize(RelativeSize),
	LengthPercentage(LengthPercentage),
}

impl FontSize {
	pub fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<FontSize, ParseError<'i>> {
		input
			.try_parse(|input| {
				let size = AbsoluteSize::parse(input)?;
				Ok(FontSize::AbsoluteSize(size))
			})
			.or_else(|_err: ParseError<'i>| {
				let size = input.try_parse(|input| RelativeSize::parse(input))?;
				Ok(FontSize::RelativeSize(size))
			})
			.or_else(|_err: ParseError<'i>| {
				let value = LengthPercentage::parse(input)?;
				Ok(FontSize::LengthPercentage(value))
			})
	}

	pub fn to_computed_value<'a>(&self, context: &'a mut StyleContext) -> CSSFloat {
		match self {
			FontSize::AbsoluteSize(value) => value.to_computed_value(),
			FontSize::RelativeSize(_) => not_supported!(),
			FontSize::LengthPercentage(value) => match value {
				LengthPercentage::Length(length) => length.to_computed_value(context),
				LengthPercentage::Percentage(percentage) => {
					context.parent_style.get_font_size() * percentage.to_value(&(0.0..1.0))
				},
			},
		}
	}
}

impl ToCss for FontSize {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		match self {
			FontSize::AbsoluteSize(value) => value.to_css(dest),
			FontSize::RelativeSize(value) => value.to_css(dest),
			FontSize::LengthPercentage(value) => value.to_css(dest),
		}
	}
}

pub fn initial_value() -> FontSize {
	FontSize::AbsoluteSize(AbsoluteSize::Medium)
}

pub fn cascade_property<'a>(declaration: Option<&PropertyDeclaration>, context: &'a mut StyleContext) {
	let computed_value = computed::from_inherited_property!(
		declaration,
		context.parent_style.get_font_size(),
		initial_value().to_computed_value(context),
		LonghandId::FontSize,
		PropertyDeclaration::FontSize(value) => value.to_computed_value(context)
	);
	context.computed_values.set_font_size(computed_value);
}

pub fn parse_declared<'i, 't>(
	_context: &ParserContext,
	input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
	FontSize::parse(input).map(PropertyDeclaration::FontSize)
}
