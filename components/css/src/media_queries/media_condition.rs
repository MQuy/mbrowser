use std::fmt::Write;

use cssparser::{match_ignore_ascii_case, Parser, _cssparser_internal_to_lowercase};

use super::media_feature_expression::MediaFeatureExpression;
use crate::css_writer::ToCss;
use crate::parser::ParseError;
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;

/// A binary `and` or `or` operator.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
#[allow(missing_docs)]
pub enum Operator {
    And,
    Or,
}

impl ToCss for Operator {
    fn to_css<W>(&self, dest: &mut crate::css_writer::CssWriter<W>) -> core::fmt::Result
    where
        W: Write,
    {
        match self {
            Operator::And => dest.write_str("and"),
            Operator::Or => dest.write_str("or"),
        }
    }
}

/// Represents a media condition.
#[derive(Clone, Debug, PartialEq)]
pub enum MediaCondition {
    /// A simple media feature expression, implicitly parenthesized.
    Feature(MediaFeatureExpression),
    /// A negation of a condition.
    Not(Box<MediaCondition>),
    /// A set of joint operations.
    Operation(Vec<MediaCondition>, Operator),
    /// A condition wrapped in parenthesis.
    InParens(Box<MediaCondition>),
    /// A future expansion
    GeneralEnclosed(String),
}

enum AllowedOperator {
    All,
    And,
    Or,
}

impl AllowedOperator {
    pub fn is_ok(&self, op: Operator) -> bool {
        match self {
            AllowedOperator::All => true,
            AllowedOperator::And if op == Operator::And => true,
            AllowedOperator::Or if op == Operator::Or => true,
            _ => false,
        }
    }
}

impl MediaCondition {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        MediaCondition::parse_internal(context, input, AllowedOperator::All)
    }

    pub fn parse_without_or<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        MediaCondition::parse_internal(context, input, AllowedOperator::And)
    }

    fn parse_internal<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
        allowed_op: AllowedOperator,
    ) -> Result<Self, ParseError<'i>> {
        input
            .try_parse(|input| MediaCondition::parse_media_not(context, input))
            .or_else(|_err| {
                input.try_parse(|input| {
                    MediaCondition::parse_media_and_or(context, input, allowed_op)
                })
            })
    }

    fn parse_media_not<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        let ident = input.expect_ident()?.to_string();
        if ident == "not" {
            let media_condition = MediaCondition::parse_in_parens(context, input)?;
            Ok(MediaCondition::Not(Box::new(media_condition)))
        } else {
            Err(input.new_custom_error(StyleParseErrorKind::MediaQueryExpectedToken))
        }
    }

    fn parse_media_and_or<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
        allowed_op: AllowedOperator,
    ) -> Result<Self, ParseError<'i>> {
        let media = MediaCondition::parse_in_parens(context, input)?;

        let (mut extras, op) =
            MediaCondition::parse_media_and_or_repeated(context, input, allowed_op)?;
        if extras.len() == 0 {
            Ok(media)
        } else {
            extras.insert(0, media);
            Ok(MediaCondition::Operation(extras, op.unwrap()))
        }
    }

    fn parse_media_and_or_repeated<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
        allowed_ops: AllowedOperator,
    ) -> Result<(Vec<Self>, Option<Operator>), ParseError<'i>> {
        let mut extras = vec![];
        let mut expected_op: Option<Operator> = None;
        loop {
            let result = input.try_parse(|input| {
                let location = input.current_source_location();
                let ident = input.expect_ident()?;
                let op = match_ignore_ascii_case! { ident,
                    "and" => Operator::And,
                    "or" => Operator::Or,
                    _ => return Err(location.new_custom_error(StyleParseErrorKind::UnexpectedValue(ident.clone()))),
                };
                if let Some(expected_op) = expected_op {
                    if expected_op != op {
                        return Err(location.new_custom_error(StyleParseErrorKind::UnexpectedValue(ident.clone())));
                    }
                } else if allowed_ops.is_ok(op) {
                    expected_op = Some(op);
                } else {
                    return Err(location.new_custom_error(StyleParseErrorKind::UnexpectedValue(ident.clone())));
                }
                MediaCondition::parse_in_parens(context, input)
            });
            if let Ok(media) = result {
                extras.push(media);
            } else {
                break;
            }
        }
        Ok((extras, expected_op))
    }

    fn parse_in_parens<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        input.expect_parenthesis_block()?;
        input.parse_nested_block(|input| {
            let media_condition = input
                .try_parse(|input| MediaFeatureExpression::parse(context, input))
                .or_else(|_err| {
                    input
                        .try_parse(|input| MediaCondition::parse(context, input))
                        .or_else(|_err| -> Result<Self, ParseError<'i>> {
                            let value = parse_general_enclosed(input)?;
                            Ok(MediaCondition::GeneralEnclosed(value))
                        })
                })?;
            Ok(MediaCondition::InParens(Box::new(media_condition)))
        })
    }
}

impl ToCss for MediaCondition {
    fn to_css<W>(&self, dest: &mut crate::css_writer::CssWriter<W>) -> core::fmt::Result
    where
        W: std::fmt::Write,
    {
        match self {
            MediaCondition::Feature(feature) => feature.to_css(dest),
            MediaCondition::Not(media_condition) => {
                dest.write_str("not ")?;
                media_condition.to_css(dest)
            },
            MediaCondition::Operation(medias, op) => medias
                .iter()
                .enumerate()
                .map(|(index, media)| {
                    if index > 0 {
                        dest.write_char(' ')?;
                        op.to_css(dest)?;
                        dest.write_char(' ')?;
                    }
                    media.to_css(dest)
                })
                .collect(),
            MediaCondition::InParens(media_condition) => {
                dest.write_char('(')?;
                media_condition.to_css(dest)?;
                dest.write_char(')')
            },
            MediaCondition::GeneralEnclosed(value) => dest.write_str(value),
        }
    }
}

/// <https://drafts.csswg.org/css-syntax-3/#typedef-any-value>
pub fn consume_any_value<'i, 't>(input: &mut Parser<'i, 't>) -> Result<String, ParseError<'i>> {
    let pos = input.position();
    input
        .expect_no_error_token()
        .map_err(|err| -> ParseError<'i> { err.into() })?;
    Ok(input.slice_from(pos).to_owned())
}

pub fn parse_general_enclosed<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> Result<String, ParseError<'i>> {
    let position = input.position();
    input
        .try_parse(|input| {
            input.expect_function()?;
            input.parse_nested_block(|input| consume_any_value(input))
        })
        .or_else(|_err| {
            input.expect_ident()?;
            consume_any_value(input)
        })?;
    Ok(input.slice_from(position).to_owned())
}
