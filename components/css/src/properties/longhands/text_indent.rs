use cssparser::{Parser, ToCss};

use crate::css_writer::write_elements;
use crate::parser::{parse_in_any_order, parse_item_if_missing, ParseError};
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::length::LengthPercentage;

/// https://drafts.csswg.org/css-text/#text-indent-property
#[derive(Clone, Debug)]
pub struct TextIndent {
	indent: LengthPercentage,
	hanging: bool,
	each_line: bool,
}

impl TextIndent {
	pub fn parse<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		let mut indent = None;
		let mut hanging = None;
		let mut each_line = None;
		parse_in_any_order(
			input,
			&mut [
				&mut |input| {
					parse_item_if_missing(input, &mut indent, &mut |_, input| {
						LengthPercentage::parse(context, input)
					})
				},
				&mut |input| {
					parse_item_if_missing(input, &mut hanging, &mut |_, input| {
						input.expect_ident_matching("hanging")?;
						Ok(true)
					})
				},
				&mut |input| {
					parse_item_if_missing(input, &mut each_line, &mut |_, input| {
						input.expect_ident_matching("each-line")?;
						Ok(true)
					})
				},
			],
		);
		if let Some(indent) = indent {
			Ok(TextIndent {
				indent,
				hanging: hanging.map_or(false, |_v| true),
				each_line: each_line.map_or(false, |_v| true),
			})
		} else {
			Err(input.new_custom_error(StyleParseErrorKind::UnspecifiedError))
		}
	}
}

impl ToCss for TextIndent {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		let indent = Some(self.indent.to_css_string());
		let hanging = if self.hanging { Some("hanging") } else { None };
		let each_line = if self.each_line {
			Some("each-line")
		} else {
			None
		};
		write_elements(dest, &[indent.as_deref(), hanging, each_line], ' ')
	}
}
pub fn parse_declared<'i, 't>(
	context: &ParserContext,
	input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
	TextIndent::parse(context, input).map(PropertyDeclaration::TextIndent)
}
