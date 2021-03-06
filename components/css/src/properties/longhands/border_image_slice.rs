use cssparser::{Parser, ToCss};

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::generics::length::Rect;
use crate::values::specified::number::NonNegativeNumberOrPercentage;

/// https://drafts.csswg.org/css-backgrounds/#the-border-image-slice
#[derive(Clone, Debug)]
pub struct BorderImageSlice {
	pub offsets: Rect<NonNegativeNumberOrPercentage>,
	pub fill: bool,
}

impl BorderImageSlice {
	pub fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i>> {
		let mut fill = input.try_parse(|input| input.expect_ident_matching("fill")).is_ok();
		let offsets = Rect::parse_with(input, |input| NonNegativeNumberOrPercentage::parse(input))?;

		if !fill {
			fill = input.try_parse(|input| input.expect_ident_matching("fill")).is_ok();
		}

		Ok(BorderImageSlice { fill, offsets })
	}
}

impl ToCss for BorderImageSlice {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		dest.write_fmt(format_args!(
			"{}{}",
			self.offsets.to_css_string(),
			if self.fill { " fill" } else { "" }
		))
	}
}

pub fn parse_declared<'i, 't>(
	_context: &ParserContext,
	input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
	BorderImageSlice::parse(input).map(PropertyDeclaration::BorderImageSlice)
}
