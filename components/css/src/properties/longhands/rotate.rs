use cssparser::{match_ignore_ascii_case, Parser, Token, _cssparser_internal_to_lowercase};

use crate::parser::{parse_in_any_order, parse_item_if_missing, ParseError};
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::number::Number;
use crate::values::specified::angle::Angle;

#[derive(Clone)]
#[repr(C, u8)]
pub enum Rotate {
    None,
    Rotate(Angle),
    Rotate3D(Number, Number, Number, Angle),
}

impl Rotate {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Rotate, ParseError<'i>> {
        input
            .try_parse(|input| {
                input.expect_ident_matching("none")?;
                Ok(Rotate::None)
            })
            .or_else(|_err: ParseError<'i>| {
                input.try_parse(|input| {
                    let angle = Angle::parse(context, input)?;
                    Ok(Rotate::Rotate(angle))
                })
            })
            .or_else(|_err: ParseError<'i>| {
                let mut angle = None;
                let mut coordinate = None;
                parse_in_any_order(
                    input,
                    &mut [
                        &mut |input| {
                            parse_item_if_missing(input, &mut angle, |_, input| {
                                Angle::parse(context, input)
                            })
                        },
                        &mut |input| {
                            parse_item_if_missing(input, &mut coordinate, |_, input| {
                                let location = input.current_source_location();
                                let token = input.next()?.clone();
                                let coordinate = match &token {
                                    Token::Ident(ident) => match_ignore_ascii_case! { ident,
                                        "x" => (Number::new(1.0), Number::new(0.0), Number::new(0.0)),
                                        "y" => (Number::new(0.0), Number::new(1.0), Number::new(0.0)),
                                        "z" => (Number::new(0.0), Number::new(0.0), Number::new(1.0)),
                                        _ => return Err(location.new_custom_error(
                                            StyleParseErrorKind::UnexpectedToken(token.clone()),
                                        ))
                                    },
                                    Token::Number { value: x, .. } => {
                                        let y = Number::parse(context, input)?;
                                        let z = Number::parse(context, input)?;
                                        (Number::new(x.clone()), y, z)
                                    },
                                    _ => {
                                        return Err(location.new_custom_error(
                                            StyleParseErrorKind::UnexpectedToken(token.clone()),
                                        ))
                                    },
                                };
                                Ok(coordinate)
                            })
                        },
                    ],
                );
                if let (Some((x, y, z)), Some(angle)) = (coordinate, angle) {
                    Ok(Rotate::Rotate3D(x, y, z, angle))
                }
                else {
                    Err(input.new_custom_error(
                        StyleParseErrorKind::UnspecifiedError,
                    ))
                }
            })
    }
}

pub fn parse_declared<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
    Rotate::parse(context, input).map(PropertyDeclaration::Rotate)
}
