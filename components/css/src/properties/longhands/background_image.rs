use cssparser::Parser;

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::image::Image;

#[derive(Clone)]
pub struct BackgroundImage {
    images: Vec<Image>,
}

impl BackgroundImage {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        let images = input.parse_comma_separated(|input| Image::parse(context, input))?;
        Ok(BackgroundImage { images })
    }
}

pub fn parse_declared<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
    BackgroundImage::parse(context, input).map(PropertyDeclaration::BackgroundImage)
}
