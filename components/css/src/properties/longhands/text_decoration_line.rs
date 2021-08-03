use cssparser::Parser;

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
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
        /// Only set by presentation attributes
        ///
        /// Setting this will mean that text-decorations use the color
        /// specified by `color` in quirks mode.
        ///
        /// For example, this gives <a href=foo><font color="red">text</font></a>
        /// a red text decoration
        #[cfg(feature = "gecko")]
        const COLOR_OVERRIDE = 0x10;
    }
}

pub fn parse<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<TextDecorationLine, ParseError<'i>> {
    panic!()
}

pub fn parse_declared<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
    parse(context, input).map(PropertyDeclaration::TextDecorationLine)
}
