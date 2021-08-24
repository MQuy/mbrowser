use cssparser::{Parser, ToCss, _cssparser_internal_to_lowercase, match_ignore_ascii_case};

use crate::css_writer::write_elements;
use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;

bitflags! {
    #[repr(C)]
    /// https://drafts.csswg.org/css-text-decor/#text-decoration-line-property
    pub struct TextDecorationLine: u8 {
        /// No text decoration line is specified.
        const NONE = 0;
        /// underline
        const UNDERLINE = 1 << 0;
        /// overline
        const OVERLINE = 1 << 1;
        /// line-through
        const LINE_THROUGH = 1 << 2;
        /// blink
        const BLINK = 1 << 3;
    }
}

impl TextDecorationLine {
    pub fn parse<'i, 't>(
        _context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<TextDecorationLine, ParseError<'i>> {
        input
            .try_parse(|input| {
                input.expect_ident_matching("none")?;
                Ok(TextDecorationLine::NONE)
            })
            .or_else(|_err: ParseError<'i>| {
                let mut bits = 0;
                loop {
                    if input.try_parse(|input|  -> Result<(), ParseError<'i>> {
                        let location = input.current_source_location();
                        let ident = input.expect_ident()?;
                        bits = bits |  match_ignore_ascii_case! { ident,
                            "underline" => TextDecorationLine::UNDERLINE.bits,
                            "overline" => TextDecorationLine::OVERLINE.bits,
                            "line-through" => TextDecorationLine::LINE_THROUGH.bits,
                            "blink" => TextDecorationLine::BLINK.bits,
                            _ => return Err(location.new_custom_error(StyleParseErrorKind::UnexpectedValue(ident.clone()))),
                        };
                        Ok(())
                    }).is_err() {
                        break;
                    }
                };
                if bits == 0 {
                    Err(input.new_custom_error(StyleParseErrorKind::UnspecifiedError))
                } else {
                    Ok(TextDecorationLine { bits})
                }
            })
    }
}

impl ToCss for TextDecorationLine {
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

        let none = convert_to(self.bits, TextDecorationLine::NONE.bits, "none");
        let underline = convert_to(self.bits, TextDecorationLine::NONE.bits, "underline");
        let overline = convert_to(self.bits, TextDecorationLine::NONE.bits, "overline");
        let line_through = convert_to(self.bits, TextDecorationLine::NONE.bits, "line-through");
        let blink = convert_to(self.bits, TextDecorationLine::NONE.bits, "blink");
        write_elements(dest, &[none, underline, overline, line_through, blink], ' ')
    }
}

pub fn parse_declared<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
    TextDecorationLine::parse(context, input).map(PropertyDeclaration::TextDecorationLine)
}
