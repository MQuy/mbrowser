use std::fmt::Write;

use cssparser::{BasicParseError, Parser, Token};

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
    Operation(Box<MediaCondition>, Operator, Box<MediaCondition>),
    /// A condition wrapped in parenthesis.
    InParens(Box<MediaCondition>),
    /// A future expansion
    GeneralEnclosed(String),
}

impl MediaCondition {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        input
            .try_parse(|input| MediaCondition::parse_media_not(context, input))
            .or_else(|_err| {
                input
                    .try_parse(|input| MediaCondition::parse_media_and_or(context, input))
                    .or_else(|_err| MediaCondition::parse_in_parens(context, input))
            })
    }

    pub fn parse_media_not<'i, 't>(
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

    pub fn parse_media_and_or<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        let left = Box::new(MediaCondition::parse_in_parens(context, input)?);
        let operator = match input.expect_ident()?.to_string() {
            op if op == "and" => Operator::And,
            op if op == "or" => Operator::Or,
            _ => return Err(input.new_custom_error(StyleParseErrorKind::MediaQueryExpectedToken)),
        };
        let right = Box::new(MediaCondition::parse_in_parens(context, input)?);
        Ok(MediaCondition::Operation(left, operator, right))
    }

    pub fn parse_in_parens<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        input.expect_parenthesis_block()?;
        input.parse_nested_block(|input| {
            let media_condition = input
                .try_parse(|input| MediaCondition::parse(context, input))
                .or_else(|_err| {
                    input
                        .try_parse(|input| MediaFeatureExpression::parse(context, input))
                        .or_else(|_err| -> Result<Self, ParseError<'i>> {
                            let value = MediaCondition::parse_general_enclosed(input)?;
                            Ok(MediaCondition::GeneralEnclosed(value))
                        })
                })?;
            Ok(MediaCondition::InParens(Box::new(media_condition)))
        })
    }

    pub fn parse_general_enclosed<'i, 't>(
        input: &mut Parser<'i, 't>,
    ) -> Result<String, ParseError<'i>> {
        input
            .try_parse(|input| {
                input.expect_function()?;
                input.parse_nested_block(|input| consume_any_value(input))
            })
            .or_else(|_err| {
                input.expect_ident()?;
                consume_any_value(input)
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
            MediaCondition::Operation(left, op, right) => {
                left.to_css(dest)?;
                dest.write_char(' ')?;
                op.to_css(dest)?;
                dest.write_char(' ')?;
                right.to_css(dest)
            },
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
