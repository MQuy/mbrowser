use cssparser::{match_ignore_ascii_case, Parser, ToCss, _cssparser_internal_to_lowercase};

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::specified::position::{BaselinePosition, OverflowPosition, SelfPosition};

/// https://drafts.csswg.org/css-align-3/#align-items-property
#[derive(Clone, Debug)]
pub enum AlignItems {
	Normal,
	Stretch,
	Baseline(BaselinePosition),
	Overflow(Option<OverflowPosition>, SelfPosition),
}

impl AlignItems {
	pub fn parse<'i, 't>(context: &ParserContext, input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i>> {
		input
			.try_parse(|input| {
				let location = input.current_source_location();
				let ident = input.expect_ident()?;
				Ok(match_ignore_ascii_case! { ident,
					"normal" => AlignItems::Normal,
					"stretch" => AlignItems::Stretch,
					_ => return Err(location.new_custom_error(StyleParseErrorKind::UnexpectedValue(ident.clone())))
				})
			})
			.or_else(|_err: ParseError<'i>| {
				input.try_parse(|input| {
					let baseline = BaselinePosition::parse(context, input)?;
					Ok(AlignItems::Baseline(baseline))
				})
			})
			.or_else(|_err: ParseError<'i>| {
				let overflow = input.try_parse(|input| OverflowPosition::parse(input)).ok();
				let content = SelfPosition::parse(input)?;
				Ok(AlignItems::Overflow(overflow, content))
			})
	}
}

impl ToCss for AlignItems {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		match self {
			AlignItems::Normal => dest.write_str("normal"),
			AlignItems::Stretch => dest.write_str("stretch"),
			AlignItems::Baseline(value) => value.to_css(dest),
			AlignItems::Overflow(overflow, content) => dest.write_fmt(format_args!(
				"{}{}",
				overflow
					.as_ref()
					.map_or("".to_string(), |v| std::format!("{} ", v.to_css_string())),
				content.to_css_string()
			)),
		}
	}
}

pub fn parse_declared<'i, 't>(
	context: &ParserContext,
	input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
	AlignItems::parse(context, input).map(PropertyDeclaration::AlignItems)
}
