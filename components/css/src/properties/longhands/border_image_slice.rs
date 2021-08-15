use cssparser::Parser;

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::length::{NonNegativeLengthPercentage, Rect};

#[derive(Clone)]
pub struct BorderImageSlice {
    pub offsets: Rect<NonNegativeLengthPercentage>,
    pub fill: bool,
}

impl BorderImageSlice {
    /// https://drafts.csswg.org/css-backgrounds/#the-border-image-slice
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        let mut fill = input
            .try_parse(|input| input.expect_ident_matching("fill"))
            .is_ok();
        let offsets = Rect::parse_with(input, |input| {
            NonNegativeLengthPercentage::parse(context, input)
        })?;

        if !fill {
            fill = input
                .try_parse(|input| input.expect_ident_matching("fill"))
                .is_ok();
        }

        Ok(BorderImageSlice { fill, offsets })
    }
}

pub fn parse_declared<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
    BorderImageSlice::parse(context, input).map(PropertyDeclaration::BorderImageSlice)
}
