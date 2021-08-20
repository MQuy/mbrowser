use cssparser::Parser;

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::image::Image;

#[derive(Clone)]
pub enum BgImage {
    None,
    Image(Image),
}

impl BgImage {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        input
            .try_parse(|input| {
                input.expect_ident_matching("none")?;
                Ok(BgImage::None)
            })
            .or_else(|_err: ParseError<'i>| {
                let image = Image::parse(context, input)?;
                Ok(BgImage::Image(image))
            })
    }
}

#[derive(Clone)]
pub struct BackgroundImage {
    images: Vec<BgImage>,
}

impl BackgroundImage {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        let images = input.parse_comma_separated(|input| BgImage::parse(context, input))?;
        Ok(BackgroundImage { images })
    }
}

pub fn parse_declared<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
    BackgroundImage::parse(context, input).map(PropertyDeclaration::BackgroundImage)
}
