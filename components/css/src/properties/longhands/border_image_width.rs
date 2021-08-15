use cssparser::Parser;

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::length::{NonNegativeLengthPercentageNumberOrAuto, Rect};

pub type BorderImageWidth = Rect<NonNegativeLengthPercentageNumberOrAuto>;

impl BorderImageWidth {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        Rect::parse_with(input, |input| {
            NonNegativeLengthPercentageNumberOrAuto::parse(context, input)
        })
    }
}

pub fn parse_declared<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
    BorderImageWidth::parse(context, input).map(PropertyDeclaration::BorderImageWidth)
}
