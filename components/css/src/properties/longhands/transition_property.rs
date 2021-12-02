use cssparser::{Parser, ToCss};

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::CustomIdent;

#[derive(Clone, Debug)]
pub enum SingleTransitionProperty {
	All,
	Ident(CustomIdent),
}

impl SingleTransitionProperty {
	pub fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i>> {
		input
			.try_parse(|input| {
				input.expect_ident_matching("all")?;
				Ok(SingleTransitionProperty::All)
			})
			.or_else(|_err: ParseError<'i>| {
				let ident = CustomIdent::parse(input)?;
				Ok(SingleTransitionProperty::Ident(ident))
			})
	}
}

impl ToCss for SingleTransitionProperty {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		match self {
			SingleTransitionProperty::All => dest.write_str("all"),
			SingleTransitionProperty::Ident(value) => value.to_css(dest),
		}
	}
}

/// https://drafts.csswg.org/css-transitions/#transition-property-property
#[derive(Clone, Debug)]
pub struct TransitionProperty {
	properties: Vec<SingleTransitionProperty>,
}

impl TransitionProperty {
	pub fn parse<'i, 't>(
		_context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<TransitionProperty, ParseError<'i>> {
		input
			.try_parse(|input| {
				input.expect_ident_matching("none")?;
				Ok(TransitionProperty { properties: vec![] })
			})
			.or_else(|_err: ParseError<'i>| {
				let properties = input.parse_comma_separated(|input| SingleTransitionProperty::parse(input))?;
				Ok(TransitionProperty { properties })
			})
	}
}

impl ToCss for TransitionProperty {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		if self.properties.len() == 0 {
			dest.write_str("none")
		} else {
			let values: Vec<String> = self.properties.iter().map(|v| v.to_css_string()).collect();
			dest.write_str(&values.join(", "))
		}
	}
}

pub fn parse_declared<'i, 't>(
	context: &ParserContext,
	input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
	TransitionProperty::parse(context, input).map(PropertyDeclaration::TransitionProperty)
}
