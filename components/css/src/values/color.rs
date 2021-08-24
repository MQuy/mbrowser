use cssparser::{match_ignore_ascii_case, Parser, ToCss, Token, _cssparser_internal_to_lowercase};

use super::number::{Number, NumberOrPercentage};
use super::percentage::Percentage;
use super::specified::angle::Angle;
use super::Ident;
use crate::parser::{parse_repeated, ParseError};
use crate::properties::declaration::property_keywords_impl;
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;

#[derive(Clone, PartialEq)]
#[repr(C)]
pub struct RGBA {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl RGBA {
    pub fn from_hex(hex: &str) -> Option<RGBA> {
        if !hex.is_ascii() {
            return None;
        }

        let (chunk_size, default_alpha) = match hex.len() {
            6 => (2, Some(255)),
            8 => (2, None),
            3 => (1, Some(255)),
            4 => (1, None),
            _ => return None,
        };
        let (red, green, blue, alpha) = RGBA::convert_hex_to_rgba(hex, chunk_size, default_alpha);
        Some(RGBA {
            red,
            green,
            blue,
            alpha,
        })
    }

    fn convert_hex_to_rgba(
        hex: &str,
        chunk_size: usize,
        default_alpha: Option<u8>,
    ) -> (u8, u8, u8, u8) {
        let decimals = RGBA::convert_hex_to_array_of_decimal(hex, chunk_size);
        let alpha = if let Some(alpha) = default_alpha {
            alpha
        } else {
            decimals[3]
        };
        (decimals[0], decimals[1], decimals[2], alpha)
    }

    fn convert_hex_to_array_of_decimal(hex: &str, chunk_size: usize) -> Vec<u8> {
        hex.as_bytes()
            .chunks(chunk_size)
            .map(|s| unsafe { ::std::str::from_utf8_unchecked(s) })
            .collect::<Vec<_>>()
            .iter()
            .map(|value| RGBA::hex_to_decimal(value) as u8)
            .collect::<Vec<_>>()
    }

    fn hex_to_decimal(hex: &str) -> usize {
        assert!(hex.is_ascii());

        let length = hex.len();
        hex.as_bytes()
            .iter()
            .enumerate()
            .fold(0, |acc, (index, char)| {
                let char = char.to_ascii_lowercase();
                let value = if char >= 48 && char <= 57 {
                    char - 48
                } else if char >= 97 && char <= 102 {
                    char - 97 + 10
                } else {
                    panic!()
                };
                acc + value as usize * usize::pow(16, (length - index - 1) as u32)
            })
    }
}

#[derive(Clone, PartialEq)]
pub struct CMYK {
    pub cyan: NumberOrPercentage,
    pub magenta: NumberOrPercentage,
    pub yellow: NumberOrPercentage,
    pub black: NumberOrPercentage,
}

impl CMYK {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        let cyan = NumberOrPercentage::parse_in_range(context, input, &(0.0..1.0))?;
        let magenta = NumberOrPercentage::parse_in_range(context, input, &(0.0..1.0))?;
        let yellow = NumberOrPercentage::parse_in_range(context, input, &(0.0..1.0))?;
        let black = NumberOrPercentage::parse_in_range(context, input, &(0.0..1.0))?;
        Ok(CMYK {
            cyan,
            magenta,
            yellow,
            black,
        })
    }

    pub fn to_rgb(&self) -> RGBA {
        fn to_float(value: &NumberOrPercentage) -> f32 {
            match value {
                NumberOrPercentage::Number(value) => value.get(),
                NumberOrPercentage::Percentage(value) => value.to_value(&(0.0..1.0)),
            }
        }

        let cyan = to_float(&self.cyan);
        let magenta = to_float(&self.magenta);
        let yellow = to_float(&self.yellow);
        let black = to_float(&self.black);

        let red = (1.0 - f32::min(1.0, cyan * (1.0 - black) + black)) * 255.0;
        let green = (1.0 - f32::min(1.0, magenta * (1.0 - black) + black)) * 255.0;
        let blue = (1.0 - f32::min(1.0, yellow * (1.0 - black) + black)) * 255.0;

        RGBA {
            red: red as u8,
            green: green as u8,
            blue: blue as u8,
            alpha: 1,
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum Hue {
    Number(Number),
    Angle(Angle),
}

impl Hue {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        input
            .try_parse(|input| {
                let number = Number::parse_in_range(context, input, 0.0, 360.0)?;
                Ok(Hue::Number(number))
            })
            .or_else(|_err: ParseError<'i>| {
                let angle = Angle::parse(context, input)?;
                Ok(Hue::Angle(angle))
            })
    }
}

#[derive(Clone, PartialEq)]
pub enum SystemColor {
    Canvas,
    CanvasText,
    LinkText,
    VisitedText,
    ActiveText,
    ButtonFace,
    ButtonText,
    ButtonBorder,
    Field,
    FieldText,
    Highlight,
    HighlightText,
    Mark,
    MarkText,
    GrayText,
}

property_keywords_impl! { SystemColor,
    SystemColor::Canvas, "canvas",
    SystemColor::CanvasText, "canvastext",
    SystemColor::LinkText, "linktext",
    SystemColor::VisitedText, "visitedtext",
    SystemColor::ActiveText, "activetext",
    SystemColor::ButtonFace, "buttonface",
    SystemColor::ButtonText, "buttontext",
    SystemColor::ButtonBorder, "buttonborder",
    SystemColor::Field, "field",
    SystemColor::FieldText, "fieldtext",
    SystemColor::Highlight, "highlight",
    SystemColor::HighlightText, "highlighttext",
    SystemColor::Mark, "mark",
    SystemColor::MarkText, "marktext",
    SystemColor::GrayText, "graytext",
}

pub struct NamedColor {
    pub name: &'static str,
    pub color: RGBA,
}

impl NamedColor {
    pub fn search(name: &str) -> Option<&NamedColor> {
        NAMED_COLORS.iter().find(|color| color.name == name)
    }
}

macro_rules! named_color {
    ($name:expr, $red:expr, $green:expr, $blue:expr) => {
        NamedColor {
            name: $name,
            color: RGBA {
                red: $red,
                green: $green,
                blue: $blue,
                alpha: 1,
            },
        }
    };
}

pub static NAMED_COLORS: [NamedColor; 148] = [
    named_color!("aliceblue", 240, 248, 255),
    named_color!("antiquewhite", 250, 235, 215),
    named_color!("aqua", 0, 255, 255),
    named_color!("aquamarine", 127, 255, 212),
    named_color!("azure", 240, 255, 255),
    named_color!("beige", 245, 245, 220),
    named_color!("bisque", 255, 228, 196),
    named_color!("black", 0, 0, 0),
    named_color!("blanchedalmond", 255, 235, 205),
    named_color!("blue", 0, 0, 255),
    named_color!("blueviolet", 138, 43, 226),
    named_color!("brown", 165, 42, 42),
    named_color!("burlywood", 222, 184, 135),
    named_color!("cadetblue", 95, 158, 160),
    named_color!("chartreuse", 127, 255, 0),
    named_color!("chocolate", 210, 105, 30),
    named_color!("coral", 255, 127, 80),
    named_color!("cornflowerblue", 100, 149, 237),
    named_color!("cornsilk", 255, 248, 220),
    named_color!("crimson", 220, 20, 60),
    named_color!("cyan", 0, 255, 255),
    named_color!("darkblue", 0, 0, 139),
    named_color!("darkcyan", 0, 139, 139),
    named_color!("darkgoldenrod", 184, 134, 11),
    named_color!("darkgray", 169, 169, 169),
    named_color!("darkgreen", 0, 100, 0),
    named_color!("darkgrey", 169, 169, 169),
    named_color!("darkkhaki", 189, 183, 107),
    named_color!("darkmagenta", 139, 0, 139),
    named_color!("darkolivegreen", 85, 107, 47),
    named_color!("darkorange", 255, 140, 0),
    named_color!("darkorchid", 153, 50, 204),
    named_color!("darkred", 139, 0, 0),
    named_color!("darksalmon", 233, 150, 122),
    named_color!("darkseagreen", 143, 188, 143),
    named_color!("darkslateblue", 72, 61, 139),
    named_color!("darkslategray", 47, 79, 79),
    named_color!("darkslategrey", 47, 79, 79),
    named_color!("darkturquoise", 0, 206, 209),
    named_color!("darkviolet", 148, 0, 211),
    named_color!("deeppink", 255, 20, 147),
    named_color!("deepskyblue", 0, 191, 255),
    named_color!("dimgray", 105, 105, 105),
    named_color!("dimgrey", 105, 105, 105),
    named_color!("dodgerblue", 30, 144, 255),
    named_color!("firebrick", 178, 34, 34),
    named_color!("floralwhite", 255, 250, 240),
    named_color!("forestgreen", 34, 139, 34),
    named_color!("fuchsia", 255, 0, 255),
    named_color!("gainsboro", 220, 220, 220),
    named_color!("ghostwhite", 248, 248, 255),
    named_color!("gold", 255, 215, 0),
    named_color!("goldenrod", 218, 165, 32),
    named_color!("gray", 128, 128, 128),
    named_color!("green", 0, 128, 0),
    named_color!("greenyellow", 173, 255, 47),
    named_color!("grey", 128, 128, 128),
    named_color!("honeydew", 240, 255, 240),
    named_color!("hotpink", 255, 105, 180),
    named_color!("indianred", 205, 92, 92),
    named_color!("indigo", 75, 0, 130),
    named_color!("ivory", 255, 255, 240),
    named_color!("khaki", 240, 230, 140),
    named_color!("lavender", 230, 230, 250),
    named_color!("lavenderblush", 255, 240, 245),
    named_color!("lawngreen", 124, 252, 0),
    named_color!("lemonchiffon", 255, 250, 205),
    named_color!("lightblue", 173, 216, 230),
    named_color!("lightcoral", 240, 128, 128),
    named_color!("lightcyan", 224, 255, 255),
    named_color!("lightgoldenrodyellow", 250, 250, 210),
    named_color!("lightgray", 211, 211, 211),
    named_color!("lightgreen", 144, 238, 144),
    named_color!("lightgrey", 211, 211, 211),
    named_color!("lightpink", 255, 182, 193),
    named_color!("lightsalmon", 255, 160, 122),
    named_color!("lightseagreen", 32, 178, 170),
    named_color!("lightskyblue", 135, 206, 250),
    named_color!("lightslategray", 119, 136, 153),
    named_color!("lightslategrey", 119, 136, 153),
    named_color!("lightsteelblue", 176, 196, 222),
    named_color!("lightyellow", 255, 255, 224),
    named_color!("lime", 0, 255, 0),
    named_color!("limegreen", 50, 205, 50),
    named_color!("linen", 250, 240, 230),
    named_color!("magenta", 255, 0, 255),
    named_color!("maroon", 128, 0, 0),
    named_color!("mediumaquamarine", 102, 205, 170),
    named_color!("mediumblue", 0, 0, 205),
    named_color!("mediumorchid", 186, 85, 211),
    named_color!("mediumpurple", 147, 112, 219),
    named_color!("mediumseagreen", 60, 179, 113),
    named_color!("mediumslateblue", 123, 104, 238),
    named_color!("mediumspringgreen", 0, 250, 154),
    named_color!("mediumturquoise", 72, 209, 204),
    named_color!("mediumvioletred", 199, 21, 133),
    named_color!("midnightblue", 25, 25, 112),
    named_color!("mintcream", 245, 255, 250),
    named_color!("mistyrose", 255, 228, 225),
    named_color!("moccasin", 255, 228, 181),
    named_color!("navajowhite", 255, 222, 173),
    named_color!("navy", 0, 0, 128),
    named_color!("oldlace", 253, 245, 230),
    named_color!("olive", 128, 128, 0),
    named_color!("olivedrab", 107, 142, 35),
    named_color!("orange", 255, 165, 0),
    named_color!("orangered", 255, 69, 0),
    named_color!("orchid", 218, 112, 214),
    named_color!("palegoldenrod", 238, 232, 170),
    named_color!("palegreen", 152, 251, 152),
    named_color!("paleturquoise", 175, 238, 238),
    named_color!("palevioletred", 219, 112, 147),
    named_color!("papayawhip", 255, 239, 213),
    named_color!("peachpuff", 255, 218, 185),
    named_color!("peru", 205, 133, 63),
    named_color!("pink", 255, 192, 203),
    named_color!("plum", 221, 160, 221),
    named_color!("powderblue", 176, 224, 230),
    named_color!("purple", 128, 0, 128),
    named_color!("rebeccapurple", 102, 51, 153),
    named_color!("red", 255, 0, 0),
    named_color!("rosybrown", 188, 143, 143),
    named_color!("royalblue", 65, 105, 225),
    named_color!("saddlebrown", 139, 69, 19),
    named_color!("salmon", 250, 128, 114),
    named_color!("sandybrown", 244, 164, 96),
    named_color!("seagreen", 46, 139, 87),
    named_color!("seashell", 255, 245, 238),
    named_color!("sienna", 160, 82, 45),
    named_color!("silver", 192, 192, 192),
    named_color!("skyblue", 135, 206, 235),
    named_color!("slateblue", 106, 90, 205),
    named_color!("slategray", 112, 128, 144),
    named_color!("slategrey", 112, 128, 144),
    named_color!("snow", 255, 250, 250),
    named_color!("springgreen", 0, 255, 127),
    named_color!("steelblue", 70, 130, 180),
    named_color!("tan", 210, 180, 140),
    named_color!("teal", 0, 128, 128),
    named_color!("thistle", 216, 191, 216),
    named_color!("tomato", 255, 99, 71),
    named_color!("turquoise", 64, 224, 208),
    named_color!("violet", 238, 130, 238),
    named_color!("wheat", 245, 222, 179),
    named_color!("white", 255, 255, 255),
    named_color!("whitesmoke", 245, 245, 245),
    named_color!("yellow", 255, 255, 0),
    named_color!("yellowgreen", 154, 205, 50),
];

#[derive(Clone, PartialEq)]
pub enum Color {
    CurrentColor,
    Transparent,
    RGB(RGBA),
    HSL(Hue, Percentage, Percentage, u8),
    HWB(Hue, Percentage, Percentage, u8),
    LAB(Percentage, Number, Number, u8),
    LCH(Percentage, Number, Hue, u8),
    Color(Ident, Vec<NumberOrPercentage>, u8),
    DeviceCMYK(CMYK, u8, Box<Color>),
    System(SystemColor),
}

impl Color {
    /// https://drafts.csswg.org/css-color/#color-syntax
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Color, ParseError<'i>> {
        input.try_parse(|input| {
            let location = input.current_source_location();
            let ident = input.expect_ident()?;
            Ok(match_ignore_ascii_case! { ident,
                "currentcolor" => Color::CurrentColor,
                "transparent" => Color::Transparent,
                _ => return Err(location.new_custom_error(StyleParseErrorKind::UnexpectedValue(ident.clone())))
            })
        }).or_else(|_err: ParseError<'i>| {
            input.try_parse(|input| {
                let location = input.current_source_location();
                let token = input.next()?;
                let error = Err(location.new_custom_error(StyleParseErrorKind::UnexpectedToken(token.clone())));
                let value = match token {
                    Token::Ident(ident) => NamedColor::search(ident).map_or(error, |value| Ok(value.color.clone())),
                    Token::Hash(value) => RGBA::from_hex(value).map_or(error, |v| Ok(v)),
                    _ => return Err(location.new_custom_error(StyleParseErrorKind::UnexpectedToken(token.clone())))
                }?;
                Ok(Color::RGB(value))
            })
        }).or_else(|_err: ParseError<'i>| {
            let color = input.try_parse(|input| SystemColor::parse(input))?;
            Ok(Color::System(color))
        }).or_else(|_err: ParseError<'i>| {
            let location = input.current_source_location();
            let ident = input.expect_function()?.clone();
            input.parse_nested_block(|input| {
                match_ignore_ascii_case! { &ident,
                    "rgb" | "rgba" => Color::parse_rgb(context, input),
                    "hsl" | "hsla" => Color::parse_hsl(context, input),
                    "hwb" => Color::parse_hwb(context, input),
                    "lab" => Color::parse_lab(context, input),
                    "lch" => Color::parse_lch(context, input),
                    "color" => Color::parse_color(context, input),
                    "device-cmyk" => Color::parse_device_cmyk(context, input),
                    _ => return Err(location.new_custom_error(StyleParseErrorKind::UnexpectedValue(ident.clone())))
                }
            })
        })
    }

    pub fn parse_rgb<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Color, ParseError<'i>> {
        let (red, green, blue) = input
            .try_parse(|input| -> Result<(f32, f32, f32), ParseError<'i>> {
                let red = Percentage::parse(context, input)?;
                let green = Percentage::parse(context, input)?;
                let blue = Percentage::parse(context, input)?;
                let range = 0.0..255.0;
                Ok((
                    red.to_value(&range),
                    green.to_value(&range),
                    blue.to_value(&range),
                ))
            })
            .or_else(
                |_err: ParseError<'i>| -> Result<(f32, f32, f32), ParseError<'i>> {
                    input.try_parse(|input| {
                        let red = Number::parse_in_range(context, input, 0.0, 255.0)?;
                        let green = Number::parse_in_range(context, input, 0.0, 255.0)?;
                        let blue = Number::parse_in_range(context, input, 0.0, 255.0)?;
                        Ok((red.get(), green.get(), blue.get()))
                    })
                },
            )
            .map(|(red, green, blue)| (red as u8, green as u8, blue as u8))?;

        let delimitor = input.next()?.clone();
        match delimitor {
            Token::Delim(value) if value == '/' || value == ',' => (),
            _ => {
                return Err(
                    input.new_custom_error(StyleParseErrorKind::UnexpectedToken(delimitor.clone()))
                )
            },
        };
        let alpha = Color::parse_alpha_value(context, input);
        Ok(Color::RGB(RGBA {
            red,
            green,
            blue,
            alpha,
        }))
    }

    pub fn parse_hsl<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Color, ParseError<'i>> {
        let hue = Hue::parse(context, input)?;
        let saturation = Percentage::parse(context, input)?;
        let lightness = Percentage::parse(context, input)?;
        let alpha = Color::parse_alpha_value(context, input);
        Ok(Color::HSL(hue, saturation, lightness, alpha))
    }

    pub fn parse_hwb<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Color, ParseError<'i>> {
        let hue = Hue::parse(context, input)?;
        let saturation = Percentage::parse(context, input)?;
        let lightness = Percentage::parse(context, input)?;
        let alpha = Color::parse_alpha_value(context, input);
        Ok(Color::HWB(hue, saturation, lightness, alpha))
    }

    pub fn parse_lab<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Color, ParseError<'i>> {
        let lightness = Percentage::parse(context, input)?;
        let a = Number::parse_in_range(context, input, -160.0, 160.0)?;
        let b = Number::parse_in_range(context, input, -160.0, 160.0)?;
        let alpha = Color::parse_alpha_value(context, input);
        Ok(Color::LAB(lightness, a, b, alpha))
    }

    pub fn parse_lch<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Color, ParseError<'i>> {
        let lightness = Percentage::parse(context, input)?;
        let chroma = Number::parse_in_range(context, input, 0.0, 230.0)?;
        let hue = Hue::parse(context, input)?;
        let alpha = Color::parse_alpha_value(context, input);
        Ok(Color::LCH(lightness, chroma, hue, alpha))
    }

    pub fn parse_color<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Color, ParseError<'i>> {
        let ident = input.expect_ident()?.to_string();
        let values = parse_repeated(
            input,
            &mut |input| NumberOrPercentage::parse(context, input),
            1,
        )?;
        let alpha = Color::parse_alpha_value(context, input);
        Ok(Color::Color(Ident(ident), values, alpha))
    }

    pub fn parse_device_cmyk<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Color, ParseError<'i>> {
        let cmyk = CMYK::parse(context, input)?;
        let alpha = Color::parse_alpha_value(context, input);
        let color =
            Color::parse(context, input).map_or_else(|_err| Color::RGB(cmyk.to_rgb()), |v| v);
        Ok(Color::DeviceCMYK(cmyk, alpha, Box::new(color)))
    }

    fn parse_alpha_value<'i, 't>(context: &ParserContext, input: &mut Parser<'i, 't>) -> u8 {
        input
            .try_parse(|input| -> Result<f32, ParseError<'i>> {
                input.expect_delim('/')?;
                input
                    .try_parse(|input| -> Result<f32, ParseError<'i>> {
                        let number = Number::parse_in_range(context, input, 0.0, 1.0)?;
                        Ok(number.get() * 255.0)
                    })
                    .or_else(|_err: ParseError<'i>| -> Result<f32, ParseError<'i>> {
                        input.try_parse(|input| {
                            let value = Percentage::parse(context, input)?;
                            Ok(value.to_value(&(0.0..255.0)))
                        })
                    })
            })
            .map_or(255, |v| v as u8)
    }
}
