use cssparser::Parser;

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::layout::LineStyle;

#[derive(Clone)]
#[repr(C, u8)]
pub enum OutlineStyle {
    Auto,
    BorderStyle(LineStyle),
}

impl OutlineStyle {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<OutlineStyle, ParseError<'i>> {
        input
            .try_parse(|input| {
                input.expect_ident_matching("auto")?;
                Ok(OutlineStyle::Auto)
            })
            .or_else(|_err: ParseError<'i>| {
                let style = LineStyle::parse(input)?;
                if style == LineStyle::Hidden {
                    return Err(input
                        .new_custom_error(StyleParseErrorKind::UnexpectedValue("hidden".into())));
                }
                Ok(OutlineStyle::BorderStyle(style))
            })
    }
}

pub fn parse_declared<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
    OutlineStyle::parse(context, input).map(PropertyDeclaration::OutlineStyle)
}
