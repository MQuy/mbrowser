use cssparser::{Parser, ToCss, Token, _cssparser_internal_to_lowercase, match_ignore_ascii_case};

use super::color::Color;
use super::layout::Resolution;
use super::length::{LengthPercentage, NonNegativeLength};
use super::percentage::Percentage;
use super::specified::angle::{Angle, AnglePercentage};
use super::specified::position::{HorizontalPosition, Position, VerticalPosition};
use super::url::CssUrl;
use super::Ident;
use crate::parser::{parse_in_any_order, parse_item_if_missing, ParseError};
use crate::properties::declaration::property_keywords_impl;
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;

#[derive(Clone)]
pub enum ImageDirection {
    Ltr,
    Rtl,
}

property_keywords_impl! { ImageDirection,
    ImageDirection::Ltr, "ltr",
    ImageDirection::Rtl, "rtl",
}

#[derive(Clone)]
pub struct Annotation {
    tag: Option<ImageDirection>,
    src: Option<CssUrl>,
    color: Option<Color>,
}

impl Annotation {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        let tag = input
            .try_parse(|input| ImageDirection::parse(input))
            .map_or(None, |v| Some(v));
        let src = input
            .try_parse(|input| CssUrl::parse(context, input))
            .or_else(|_err: ParseError<'i>| {
                input.try_parse(|input| CssUrl::parse_string(context, input))
            })
            .ok();
        input.expect_delim(',')?;
        let color = input.try_parse(|input| Color::parse(context, input)).ok();
        Ok(Annotation { tag, src, color })
    }
}

#[derive(Clone)]
pub enum ImageReference {
    Image(Image),
    String(CssUrl),
}

impl ImageReference {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        input
            .try_parse(|input| {
                let image = Image::parse(context, input)?;
                Ok(ImageReference::Image(image))
            })
            .or_else(|_err: ParseError<'i>| {
                let url = CssUrl::parse_string(context, input)?;
                Ok(ImageReference::String(url))
            })
    }
}

#[derive(Clone)]
pub struct ImageSetOption {
    reference: ImageReference,
    resolution: Resolution,
    mime: Option<String>,
}

impl ImageSetOption {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        let reference = ImageReference::parse(context, input)?;
        let mut resolution = None;
        let mut mime = None;
        parse_in_any_order(
            input,
            &mut [
                &mut |input| {
                    parse_item_if_missing(input, &mut resolution, |_, input| {
                        Resolution::parse(context, input)
                    })
                },
                &mut |input| {
                    parse_item_if_missing(input, &mut mime, |_, input| {
                        input.expect_function_matching("type")?;
                        input.parse_nested_block(|input| {
                            let value = input.expect_string()?.to_string();
                            Ok(value)
                        })
                    })
                },
            ],
        );

        if resolution.is_none() && mime.is_none() {
            return Err(input.new_custom_error(StyleParseErrorKind::UnspecifiedError));
        }
        Ok(ImageSetOption {
            reference,
            mime,
            resolution: resolution.map_or("1x".into(), |v| v),
        })
    }
}

#[derive(Clone)]
pub enum ImageOrColor {
    Image(Image),
    Color(Color),
}

impl ImageOrColor {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        input
            .try_parse(|input| {
                let image = Image::parse(context, input)?;
                Ok(ImageOrColor::Image(image))
            })
            .or_else(|_err: ParseError<'i>| {
                let color = Color::parse(context, input)?;
                Ok(ImageOrColor::Color(color))
            })
    }
}

#[derive(Clone)]
pub struct CFImage {
    percentage: Option<Percentage>,
    fade: ImageOrColor,
}

impl CFImage {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        let mut percentage = None;
        let mut fade = None;
        parse_in_any_order(
            input,
            &mut [
                &mut |input| {
                    parse_item_if_missing(input, &mut percentage, |_, input| {
                        Percentage::parse(context, input)
                    })
                },
                &mut |input| {
                    parse_item_if_missing(input, &mut fade, |_, input| {
                        ImageOrColor::parse(context, input)
                    })
                },
            ],
        );

        if let Some(fade) = fade {
            Ok(CFImage { fade, percentage })
        } else {
            Err(input.new_custom_error(StyleParseErrorKind::UnspecifiedError))
        }
    }
}

#[derive(Clone)]
pub enum Side {
    Left,
    Right,
}

property_keywords_impl! { Side,
    Side::Left, "left",
    Side::Right, "right",
}

#[derive(Clone)]
pub enum Corner {
    Top,
    Bottom,
}

property_keywords_impl! { Corner,
    Corner::Top, "top",
    Corner::Bottom, "bottom",
}

#[derive(Clone)]
pub enum LineDirection {
    Angle(Angle),
    Side(Side),
    Corner(Corner),
}

impl LineDirection {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        input
            .try_parse(|input| {
                let angle = Angle::parse(context, input)?;
                Ok(LineDirection::Angle(angle))
            })
            .or_else(|_err: ParseError<'i>| {
                input.try_parse(|input| {
                    let side = Side::parse(input)?;
                    Ok(LineDirection::Side(side))
                })
            })
            .or_else(|_err: ParseError<'i>| {
                input.try_parse(|input| {
                    let corner = Corner::parse(input)?;
                    Ok(LineDirection::Corner(corner))
                })
            })
    }
}

#[derive(Clone)]
pub struct LinearColorStop {
    color: Color,
    length: Option<LengthPercentage>,
}

impl LinearColorStop {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        let mut color = None;
        let mut length = None;
        parse_in_any_order(
            input,
            &mut [
                &mut |input| {
                    parse_item_if_missing(input, &mut color, |_, input| {
                        Color::parse(context, input)
                    })
                },
                &mut |input| {
                    parse_item_if_missing(input, &mut length, |_, input| {
                        LengthPercentage::parse(context, input)
                    })
                },
            ],
        );

        if let Some(color) = color {
            Ok(LinearColorStop { color, length })
        } else {
            Err(input.new_custom_error(StyleParseErrorKind::UnspecifiedError))
        }
    }
}

#[derive(Clone)]
pub struct LinearColorHint<T> {
    hint: Option<T>,
    color: LinearColorStop,
}

impl<T> LinearColorHint<T> {
    pub fn parse<'i, 't, P>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
        item_parser: &mut P,
    ) -> Result<Self, ParseError<'i>>
    where
        P: FnMut(&mut Parser<'i, 't>) -> Result<T, ParseError<'i>>,
    {
        let hint = input.try_parse(|input| item_parser(input)).ok();
        let color = LinearColorStop::parse(context, input)?;
        Ok(LinearColorHint { hint, color })
    }
}

#[derive(Clone)]
pub struct ColorStopList<T> {
    starting: LinearColorStop,
    ending: Vec<LinearColorHint<T>>,
}

impl<T> ColorStopList<T> {
    /// https://drafts.csswg.org/css-images-3/#color-stop-syntax
    pub fn parse<'i, 't, P>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
        item_parser: &mut P,
    ) -> Result<Self, ParseError<'i>>
    where
        P: for<'tt> Fn(&mut Parser<'i, 'tt>) -> Result<T, ParseError<'i>>,
    {
        let starting = LinearColorStop::parse(context, input)?;
        let ending = input
            .parse_comma_separated(|input| LinearColorHint::parse(context, input, item_parser))?;
        Ok(ColorStopList { starting, ending })
    }
}

#[derive(Clone)]
pub struct LinearGradient {
    direction: LineDirection,
    color_stop: ColorStopList<LengthPercentage>,
    repeating: bool,
}

impl LinearGradient {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        let direction = input
            .try_parse(|input| LineDirection::parse(context, input))
            .map_or(LineDirection::Corner(Corner::Bottom), |v| v);
        let color_stop = ColorStopList::parse(context, input, &mut |input| {
            LengthPercentage::parse(context, input)
        })?;
        Ok(LinearGradient {
            direction,
            color_stop,
            repeating: false,
        })
    }
}

#[derive(Clone)]
pub enum EndingShape {
    Circle,
    Ellipse,
}

property_keywords_impl! { EndingShape,
    EndingShape::Circle, "circle",
    EndingShape::Ellipse, "ellipse",
}

#[derive(Clone)]
pub enum RadialSize {
    ClosestSide,
    FarthestSide,
    ClosestCorner,
    FarthestCorner,
    Length(NonNegativeLength),
}

impl RadialSize {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        input.try_parse(|input| {
            let length = NonNegativeLength::parse(context, input)?;
            Ok(RadialSize::Length(length))
        }).or_else(|_err: ParseError<'i>| {
            let location = input.current_source_location();
            let ident = input.expect_ident()?;
            Ok(match_ignore_ascii_case! { ident,
                "closest-side" => RadialSize::ClosestSide,
                "farthest-side" => RadialSize::FarthestSide,
                "closest-corner" => RadialSize::ClosestCorner,
                "farthest-corner" => RadialSize::FarthestCorner,
                _ => return Err(location.new_custom_error(StyleParseErrorKind::UnexpectedValue(ident.clone()))),
            })
        })
    }
}

#[derive(Clone)]
pub struct RadialGradient {
    end_shape: EndingShape,
    size: RadialSize,
    position: Position,
    color_stop: ColorStopList<LengthPercentage>,
    repeating: bool,
}

impl RadialGradient {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        let mut end_shape = None;
        let mut size = None;
        parse_in_any_order(
            input,
            &mut [
                &mut |input| {
                    parse_item_if_missing(input, &mut end_shape, |_, input| {
                        EndingShape::parse(input)
                    })
                },
                &mut |input| {
                    parse_item_if_missing(input, &mut size, |_, input| {
                        RadialSize::parse(context, input)
                    })
                },
            ],
        );
        let position = input
            .try_parse(|input| {
                input.expect_ident_matching("at")?;
                Position::parse(context, input)
            })
            .ok();
        input.expect_delim(',')?;
        let color_stop = ColorStopList::parse(context, input, &mut |input| {
            LengthPercentage::parse(context, input)
        })?;
        let size = size.map_or(RadialSize::FarthestCorner, |v| v);
        let end_shape = match size {
            RadialSize::Length(_) if end_shape.is_none() => EndingShape::Circle,
            _ => EndingShape::Ellipse,
        };
        Ok(RadialGradient {
            end_shape,
            size,
            color_stop,
            repeating: false,
            position: position.map_or(
                Position::new(HorizontalPosition::Center, VerticalPosition::Center),
                |v| v,
            ),
        })
    }
}

#[derive(Clone)]
pub struct ConicRadient {
    angle: Angle,
    position: Position,
    color_stop: ColorStopList<AnglePercentage>,
    repeating: bool,
}

impl ConicRadient {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        let angle = input
            .try_parse(|input| {
                input.expect_ident_matching("from")?;
                Angle::parse(context, input)
            })
            .ok();
        let position = input
            .try_parse(|input| {
                input.expect_ident_matching("at")?;
                Position::parse(context, input)
            })
            .ok();

        input.expect_delim(',')?;
        let color_stop = ColorStopList::parse(context, input, &mut |input| {
            AnglePercentage::parse(context, input)
        })?;
        Ok(ConicRadient {
            color_stop,
            angle: angle.map_or("0deg".into(), |v| v),
            position: position.map_or(
                Position::new(HorizontalPosition::Center, VerticalPosition::Center),
                |v| v,
            ),
            repeating: false,
        })
    }
}

#[derive(Clone)]
pub enum Gradient {
    Linear(LinearGradient),
    Radial(RadialGradient),
    Conic(ConicRadient),
}

#[derive(Clone)]
pub enum Image {
    Url(CssUrl),
    Image(Annotation),
    Set(Vec<ImageSetOption>),
    CrossFade(Vec<CFImage>),
    Element(Ident),
    Gradient(Gradient),
}

impl Image {
    /// https://drafts.csswg.org/css-images-4/#image-values
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        input
            .try_parse(|input| {
                let url = CssUrl::parse(context, input)?;
                Ok(Image::Url(url))
            })
            .or_else(|_err: ParseError<'i>| {
                let location = input.current_source_location();
                let name = input.expect_function()?.clone();
                input.parse_nested_block(|input| {
                    match_ignore_ascii_case! { &name,
                        "image" => Image::parse_image(context, input),
                        "image-set" => Image::parse_set(context, input),
                        "cross-fade" => Image::parse_cross_fade(context, input),
                        "element" => Image::parse_element(context, input),
                        "linear-gradient" => Image::parse_linear_gradient(context, input, false),
                        "repeating-linear-gradient" => Image::parse_linear_gradient(context, input, true),
                        "radial-gradient" => Image::parse_radial_gradient(context, input, false),
                        "repeating-radial-gradient" => Image::parse_radial_gradient(context, input, true),
                        "conic-gradient" => Image::parse_conic_gradient(context, input, false),
                        "repeating-conic-gradient" => Image::parse_conic_gradient(context, input, true),
                        _ => return Err(location.new_custom_error(StyleParseErrorKind::UnspecifiedError))
                    }
                })
            })
    }

    pub fn parse_image<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        let value = Annotation::parse(context, input)?;
        Ok(Image::Image(value))
    }

    pub fn parse_set<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        let value = input.parse_comma_separated(|input| ImageSetOption::parse(context, input))?;
        if value.len() == 0 {
            Err(input.new_custom_error(StyleParseErrorKind::UnspecifiedError))
        } else {
            Ok(Image::Set(value))
        }
    }

    pub fn parse_cross_fade<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        let value = input.parse_comma_separated(|input| CFImage::parse(context, input))?;
        if value.len() == 0 {
            Err(input.new_custom_error(StyleParseErrorKind::UnspecifiedError))
        } else {
            Ok(Image::CrossFade(value))
        }
    }

    pub fn parse_element<'i, 't>(
        _context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        let location = input.current_source_location();
        let token = input.next()?;
        let id = match token {
            Token::Hash(value) => Ident(value.to_string()),
            _ => {
                return Err(
                    location.new_custom_error(StyleParseErrorKind::UnexpectedToken(token.clone()))
                )
            },
        };
        Ok(Image::Element(id))
    }

    pub fn parse_linear_gradient<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
        repeating: bool,
    ) -> Result<Self, ParseError<'i>> {
        let mut value = LinearGradient::parse(context, input)?;
        value.repeating = repeating;
        Ok(Image::Gradient(Gradient::Linear(value)))
    }

    pub fn parse_radial_gradient<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
        repeating: bool,
    ) -> Result<Self, ParseError<'i>> {
        let mut value = RadialGradient::parse(context, input)?;
        value.repeating = repeating;
        Ok(Image::Gradient(Gradient::Radial(value)))
    }

    pub fn parse_conic_gradient<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
        repeating: bool,
    ) -> Result<Self, ParseError<'i>> {
        let mut value = ConicRadient::parse(context, input)?;
        value.repeating = repeating;
        Ok(Image::Gradient(Gradient::Conic(value)))
    }
}
