use cssparser::{Parser, ToCss};

use crate::parser::{parse_repeated, ParseError};
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::specified::transform::TransformFunction;

/// https://drafts.csswg.org/css-transforms-1/#transform-property
#[derive(Clone, Debug)]
pub struct Transform(Vec<TransformFunction>);

impl Transform {
	pub fn parse<'i, 't>(context: &ParserContext, input: &mut Parser<'i, 't>) -> Result<Transform, ParseError<'i>> {
		input
			.try_parse(|input| {
				input.expect_ident_matching("none")?;
				Ok(Transform(vec![]))
			})
			.or_else(|_err: ParseError<'i>| {
				let transforms = parse_repeated(input, &mut |input| TransformFunction::parse(context, input), 1)?;
				Ok(Transform(transforms))
			})
	}
}

impl ToCss for Transform {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		match self.0.len() {
			0 => dest.write_str("none"),
			_ => {
				let values: Vec<String> = self.0.iter().map(|v| v.to_css_string()).collect();
				dest.write_str(&values.join(" "))
			},
		}
	}
}

pub fn parse_declared<'i, 't>(
	context: &ParserContext,
	input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
	Transform::parse(context, input).map(PropertyDeclaration::Transform)
}
