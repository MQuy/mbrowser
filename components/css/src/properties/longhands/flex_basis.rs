use cssparser::{Parser, ToCss};

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::length::Size;

/// https://drafts.csswg.org/css-flexbox/#flex-basis-property
#[derive(Clone)]
pub enum FlexBasis {
	Content,
	Width(Size),
}

impl FlexBasis {
	pub fn parse<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<FlexBasis, ParseError<'i>> {
		input
			.try_parse(|input| {
				input.expect_ident_matching("content")?;
				Ok(FlexBasis::Content)
			})
			.or_else(|_err: ParseError<'i>| {
				let size = Size::parse(context, input)?;
				Ok(FlexBasis::Width(size))
			})
	}
}

impl ToCss for FlexBasis {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		match self {
			FlexBasis::Content => dest.write_str("content"),
			FlexBasis::Width(value) => value.to_css(dest),
		}
	}
}

pub fn parse_declared<'i, 't>(
	context: &ParserContext,
	input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
	FlexBasis::parse(context, input).map(PropertyDeclaration::FlexBasis)
}
