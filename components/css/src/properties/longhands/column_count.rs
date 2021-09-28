use cssparser::{Parser, ToCss};

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::specified::number::Integer;

#[derive(Clone, Debug)]
pub enum ColumnCount {
	Auto,
	Integer(Integer),
}

impl ColumnCount {
	pub fn parse<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		input
			.try_parse(|input| {
				input.expect_ident_matching("auto")?;
				Ok(ColumnCount::Auto)
			})
			.or_else(|_err: ParseError<'i>| {
				let value = Integer::parse_from(context, input, 1)?;
				Ok(ColumnCount::Integer(value))
			})
	}
}

impl ToCss for ColumnCount {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		match self {
			ColumnCount::Auto => dest.write_str("auto"),
			ColumnCount::Integer(value) => value.to_css(dest),
		}
	}
}

/// https://drafts.csswg.org/css-multicol/#cc
pub fn parse_declared<'i, 't>(
	context: &ParserContext,
	input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
	ColumnCount::parse(context, input).map(PropertyDeclaration::ColumnCount)
}
