use cssparser::{match_ignore_ascii_case, Parser, _cssparser_internal_to_lowercase};

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;

bitflags! {
    #[repr(C)]
    /// Specified keyword values for the text-decoration-line property.
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

pub fn parse_declared<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
    TextDecorationLine::parse(context, input).map(PropertyDeclaration::TextDecorationLine)
}
