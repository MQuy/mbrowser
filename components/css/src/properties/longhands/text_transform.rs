use cssparser::{match_ignore_ascii_case, Parser, ToCss, _cssparser_internal_to_lowercase};

use crate::css_writer::write_elements;
use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;

bitflags! {
	 #[repr(C)]
	pub struct TextTransformCase: u8 {
		const NONE = 0;
		const CAPITALIZE = 1 << 0;
		const UPPERCASE  = 1 << 1;
		const LOWERCASE = 1 << 2;
		const FULL_WIDTH = 1 << 3;
		const FULL_SIZE_KANA = 1 << 4;
	}
}

/// https://drafts.csswg.org/css-text/#text-transform
#[derive(Clone, Debug)]
pub enum TextTransform {
	None,
	Transform(TextTransformCase),
}

impl TextTransform {
	pub fn parse<'i, 't>(
		_context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<TextTransform, ParseError<'i>> {
		input
			.try_parse(|input| {
				input.expect_ident_matching("none")?;
				Ok(TextTransform::None)
			})
			.or_else(|_err: ParseError<'i>| {
				let mut bits = 0;
				loop {
					let ret = input.try_parse(|input| -> Result<(), ParseError<'i>> {
						let location = input.current_source_location();
						let ident = input.expect_ident()?;
						bits = bits
							| match_ignore_ascii_case! {ident,
								"capitalize" => TextTransformCase::CAPITALIZE.bits,
								"uppercase" => TextTransformCase::UPPERCASE.bits,
								"lowercase" => TextTransformCase::LOWERCASE.bits,
								"full-width" => TextTransformCase::FULL_WIDTH.bits,
								"full-size-kana" => TextTransformCase::FULL_SIZE_KANA.bits,
								_ => return Err(location.new_custom_error(StyleParseErrorKind::UnexpectedValue(ident.clone())))
							};
						Ok(())
					});
					if ret.is_err() {
						break;
					}
				}
				if bits == 0 {
					Err(input.new_custom_error(StyleParseErrorKind::UnspecifiedError))
				} else {
					Ok(TextTransform::Transform(TextTransformCase { bits }))
				}
			})
	}
}

impl ToCss for TextTransform {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		fn convert_to(origin: u8, compare: u8, text: &str) -> Option<&str> {
			if (origin & compare) != 0 {
				Some(text)
			} else {
				None
			}
		}

		match self {
			TextTransform::None => dest.write_str("none"),
			TextTransform::Transform(value) => {
				let capitalize =
					convert_to(value.bits, TextTransformCase::CAPITALIZE.bits, "capitalize");
				let uppercase =
					convert_to(value.bits, TextTransformCase::UPPERCASE.bits, "uppercase");
				let lowercase =
					convert_to(value.bits, TextTransformCase::LOWERCASE.bits, "lowercase");
				let full_width =
					convert_to(value.bits, TextTransformCase::FULL_WIDTH.bits, "full-width");
				let full_size_kana = convert_to(
					value.bits,
					TextTransformCase::FULL_SIZE_KANA.bits,
					"full-size-kana",
				);
				write_elements(
					dest,
					&[capitalize, uppercase, lowercase, full_width, full_size_kana],
					' ',
				)
			},
		}
	}
}

pub fn parse_declared<'i, 't>(
	context: &ParserContext,
	input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
	TextTransform::parse(context, input).map(PropertyDeclaration::TextTransform)
}
