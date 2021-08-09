use cssparser::{Parser, Token};

use super::media_feature_expression::MediaFeatureExpression;
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
    GeneralEnclosed,
}

impl MediaCondition {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        input
            .try_parse(|input| MediaCondition::parse_media_not(context, input))
            .or_else(|_err| MediaCondition::parse_media_and_or(context, input))
            .or_else(|_err| MediaCondition::parse_in_parens(context, input))
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
                .or_else(|_err| MediaFeatureExpression::parse(context, input))
                .or_else(|_err: cssparser::ParseError<StyleParseErrorKind>| {
                    MediaCondition::parse_general_enclosed(input)
                })?;
            Ok(MediaCondition::InParens(Box::new(media_condition)))
        })
    }

    pub fn parse_general_enclosed<'i, 't>(
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        input
            .try_parse(|input| {
                input.expect_function()?;
                input.parse_nested_block(|input| MediaCondition::parse_any_value(input))
            })
            .or_else(|err: cssparser::ParseError<StyleParseErrorKind>| {
                input.expect_ident()?;
                MediaCondition::parse_any_value(input)
            })?;
        Ok(MediaCondition::GeneralEnclosed)
    }

    fn parse_any_value<'i, 't>(input: &mut Parser<'i, 't>) -> Result<(), ParseError<'i>> {
        loop {
            let token = input.next()?;
            match token {
                Token::BadUrl(_)
                | Token::BadString(_)
                | Token::CloseParenthesis
                | Token::CloseSquareBracket
                | Token::CloseCurlyBracket => {
                    return Err(
                        input.new_custom_error(StyleParseErrorKind::MediaQueryExpectedToken)
                    );
                },
                _ => (),
            }
        }
    }
}
