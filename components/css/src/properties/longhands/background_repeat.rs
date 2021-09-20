use cssparser::{match_ignore_ascii_case, Parser, ToCss, Token, _cssparser_internal_to_lowercase};

use crate::parser::ParseError;
use crate::properties::declaration::{property_keywords_impl, PropertyDeclaration};
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::length::Pair;

#[derive(Clone, Debug)]
pub enum BackgroundRepeatKeyword {
	Repeat,
	Space,
	Round,
	NoRepeat,
}

property_keywords_impl! { BackgroundRepeatKeyword,
	BackgroundRepeatKeyword::Repeat, "repeat",
	BackgroundRepeatKeyword::Space, "space",
	BackgroundRepeatKeyword::Round, "round",
	BackgroundRepeatKeyword::NoRepeat, "no-repeat",
}

pub type RepeatStyle = Pair<BackgroundRepeatKeyword>;

/// https://drafts.csswg.org/css-backgrounds/#background-repeat
#[derive(Clone, Debug)]
pub struct BackgroundRepeat {
	repeat: Vec<RepeatStyle>,
}

impl BackgroundRepeat {
	pub fn parse<'i, 't>(
		_context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		let repeat = input.parse_comma_separated(|input| {
			input
				.try_parse(|input| {
					let horizontal = BackgroundRepeatKeyword::parse(input)?;
					let veritcal = BackgroundRepeatKeyword::parse(input)?;
					Ok(RepeatStyle::new(horizontal, veritcal))
				})
				.or_else(|_err: ParseError<'i>| {
					let location = input.current_source_location();
					let ident = input.expect_ident()?;
					Ok(match_ignore_ascii_case! { ident,
						"repeat-x" => RepeatStyle::new(
							BackgroundRepeatKeyword::Repeat,
							BackgroundRepeatKeyword::NoRepeat,
						),
						"repeat-y" => RepeatStyle::new(
							BackgroundRepeatKeyword::NoRepeat,
							BackgroundRepeatKeyword::Repeat,
						),
						"repeat" => RepeatStyle::new(
							BackgroundRepeatKeyword::Repeat,
							BackgroundRepeatKeyword::Repeat,
						),
						"space" => RepeatStyle::new(
							BackgroundRepeatKeyword::Space,
							BackgroundRepeatKeyword::Space,
						),
						"round" => RepeatStyle::new(
							BackgroundRepeatKeyword::Round,
							BackgroundRepeatKeyword::Round,
						),
						"no-repeat" => RepeatStyle::new(
							BackgroundRepeatKeyword::NoRepeat,
							BackgroundRepeatKeyword::NoRepeat,
						),
						_ => return Err(location.new_custom_error(StyleParseErrorKind::UnexpectedValue(ident.clone())))
					})
				})
		})?;
		Ok(BackgroundRepeat { repeat })
	}
}

impl ToCss for BackgroundRepeat {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		let values: Vec<String> = self.repeat.iter().map(|f| f.to_css_string()).collect();
		dest.write_str(&values.join(", "))
	}
}

pub fn parse_declared<'i, 't>(
	context: &ParserContext,
	input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
	BackgroundRepeat::parse(context, input).map(PropertyDeclaration::BackgroundRepeat)
}
