use cssparser::{Parser, ToCss};

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::specified::length::LengthOrAuto;

/// https://drafts.fxtf.org/css-masking/#clip-property
#[derive(Clone, Debug)]
pub struct Clip {
	top: LengthOrAuto,
	right: LengthOrAuto,
	bottom: LengthOrAuto,
	left: LengthOrAuto,
}

impl Clip {
	pub fn parse<'i, 't>(context: &ParserContext, input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i>> {
		input.expect_function_matching("rect")?;
		input.parse_nested_block(|input| {
			let top = LengthOrAuto::parse(context, input)?;
			input.expect_comma()?;
			let right = LengthOrAuto::parse(context, input)?;
			input.expect_comma()?;
			let bottom = LengthOrAuto::parse(context, input)?;
			input.expect_comma()?;
			let left = LengthOrAuto::parse(context, input)?;
			Ok(Clip {
				top,
				right,
				bottom,
				left,
			})
		})
	}
}

impl ToCss for Clip {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		dest.write_fmt(format_args!(
			"rect({}, {}, {}, {})",
			self.top.to_css_string(),
			self.right.to_css_string(),
			self.bottom.to_css_string(),
			self.left.to_css_string(),
		))
	}
}

pub fn parse_declared<'i, 't>(
	context: &ParserContext,
	input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
	Clip::parse(context, input).map(PropertyDeclaration::Clip)
}
