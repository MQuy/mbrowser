use cssparser::{Parser, ToCss};

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::specified::position::{
	BaselinePosition, ContentDistribution, ContentPosition, OverflowPosition,
};

/// https://drafts.csswg.org/css-align-3/#propdef-align-content
#[derive(Clone)]
pub enum AlignContent {
	Normal,
	Baseline(BaselinePosition),
	Distribution(ContentDistribution),
	Overflow(Option<OverflowPosition>, ContentPosition),
}

impl AlignContent {
	pub fn parse<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		input
			.try_parse(|input| {
				input.expect_ident_matching("normal")?;
				Ok(AlignContent::Normal)
			})
			.or_else(|_err: ParseError<'i>| {
				input.try_parse(|input| {
					let baseline = BaselinePosition::parse(context, input)?;
					Ok(AlignContent::Baseline(baseline))
				})
			})
			.or_else(|_err: ParseError<'i>| {
				input.try_parse(|input| {
					let content = ContentDistribution::parse(input)?;
					Ok(AlignContent::Distribution(content))
				})
			})
			.or_else(|_err: ParseError<'i>| {
				let overflow = input.try_parse(|input| OverflowPosition::parse(input)).ok();
				let content = ContentPosition::parse(input)?;
				Ok(AlignContent::Overflow(overflow, content))
			})
	}
}

impl ToCss for AlignContent {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		match self {
			AlignContent::Normal => dest.write_str("normal"),
			AlignContent::Baseline(value) => value.to_css(dest),
			AlignContent::Distribution(value) => value.to_css(dest),
			AlignContent::Overflow(overflow, content) => dest.write_fmt(format_args!(
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
	AlignContent::parse(context, input).map(PropertyDeclaration::AlignContent)
}
