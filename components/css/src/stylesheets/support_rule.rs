use std::fmt::Write;

use cssparser::{
    Parser, SourceLocation, _cssparser_internal_to_lowercase, match_ignore_ascii_case,
};

use super::css_rule::CssRule;
use crate::css_writer::{CssWriter, ToCss};
use crate::media_queries::media_condition::{consume_any_value, parse_general_enclosed};
use crate::parser::ParseError;
use crate::stylesheets::rule_parser::StyleParseErrorKind;

/// An [`@supports`][supports] rule.
///
/// [supports]: https://drafts.csswg.org/css-conditional-3/#at-supports
#[derive(Clone)]
pub struct SupportsRule {
    /// The parsed condition
    pub condition: SupportsCondition,
    /// Child rules
    pub rules: Vec<CssRule>,
    /// The line and column of the rule's source code.
    pub source_location: SourceLocation,
}

impl ToCss for SupportsRule {
    fn to_css<W>(&self, dest: &mut CssWriter<W>) -> core::fmt::Result
    where
        W: Write,
    {
        dest.write_str("@supports ")?;
        self.condition.to_css(dest)
    }
}

/// An @supports condition
///
/// <https://drafts.csswg.org/css-conditional-3/#at-supports>
#[derive(Clone, Debug)]
pub enum SupportsCondition {
    /// `not (condition)`
    Not(Box<SupportsCondition>),
    /// `(condition)`
    Parenthesized(Box<SupportsCondition>),
    /// `(condition) and (condition) and (condition) ..`
    And(Vec<SupportsCondition>),
    /// `(condition) or (condition) or (condition) ..`
    Or(Vec<SupportsCondition>),
    /// `property-ident: value` (value can be any tokens)
    Declaration(Declaration),
    /// `(any tokens)` or `func(any tokens)`
    GeneralEnclosed(String),
}

impl SupportsCondition {
    /// Parse a condition
    ///
    /// <https://drafts.csswg.org/css-conditional/#supports_condition>
    pub fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i>> {
        input
            .try_parse(|input| SupportsCondition::parse_not(input))
            .or_else(|_err| {
                let supports_condition = SupportsCondition::parse_in_parens(input)?;
                let location = input.current_source_location();

                let current_state = input.state();
                let result = input.expect_ident();

                if result.is_err() {
                    input.reset(&current_state);
                    Ok(supports_condition)
                } else {
                    let ident = result?;
                    let (op, wrapper) = match_ignore_ascii_case! { ident,
                        "and" => ("and", SupportsCondition::And as fn(_) -> _),
                        "or" => ("or", SupportsCondition::Or as fn(_) -> _),
                        _ => return Err(location.new_custom_error(StyleParseErrorKind::UnexpectedValue(ident.clone()))),
                    };

                    let mut supports_conditions = vec![supports_condition];
                    let supports_after_op = SupportsCondition::parse_in_parens(input)?;
                    supports_conditions.push(supports_after_op);

                    let mut leftover = SupportsCondition::parse_and_or_repeated(input, op)?;
                    supports_conditions.append(&mut leftover);

                    Ok(wrapper(supports_conditions))
                }
            })
    }

    fn parse_and_or_repeated<'i, 't>(
        input: &mut Parser<'i, 't>,
        expected_op: &str,
    ) -> Result<Vec<Self>, ParseError<'i>> {
        let mut extras = vec![];
        loop {
            let result = input.try_parse(|input| {
                let location = input.current_source_location();
                let ident = input.expect_ident()?;
                match_ignore_ascii_case! { ident,
                    "and" if expected_op == "and" => (),
                    "or" if expected_op == "or" => (),
                    _ => return Err(location.new_custom_error(StyleParseErrorKind::UnexpectedValue(ident.clone()))),
                };
                SupportsCondition::parse_in_parens(input)
            });
            if let Ok(supports_condition) = result {
                extras.push(supports_condition);
            } else {
                break;
            }
        }
        Ok(extras)
    }

    fn parse_not<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i>> {
        let location = input.current_source_location();
        let ident = input.expect_ident()?;
        match_ignore_ascii_case! { ident,
            "not" => SupportsCondition::parse_in_parens(input),
            _ => Err(location.new_custom_error(StyleParseErrorKind::UnexpectedValue(ident.clone()))),
        }
    }

    fn parse_in_parens<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i>> {
        input
            .try_parse(|input| {
                input.expect_parenthesis_block()?;
                input.parse_nested_block(|input| {
                    Ok(SupportsCondition::Parenthesized(Box::new(
                        SupportsCondition::parse(input)?,
                    )))
                })
            })
            .or_else(|_err| -> Result<Self, ParseError<'i>> {
                input
                    .try_parse(|input| SupportsCondition::parse_declaration(input))
                    .or_else(|_err| {
                        let value = parse_general_enclosed(input)?;
                        Ok(SupportsCondition::GeneralEnclosed(value))
                    })
            })
    }

    fn parse_declaration<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i>> {
        let declaration = Declaration::parse(input)?;
        Ok(SupportsCondition::Declaration(declaration))
    }
}

impl ToCss for SupportsCondition {
    fn to_css<W>(&self, dest: &mut CssWriter<W>) -> core::fmt::Result
    where
        W: Write,
    {
        fn write_for_vector<W>(
            supports: &Vec<SupportsCondition>,
            op: &str,
            dest: &mut CssWriter<W>,
        ) -> core::fmt::Result
        where
            W: Write,
        {
            supports
                .iter()
                .enumerate()
                .map(|(index, support)| {
                    if index != 0 {
                        dest.write_fmt(format_args!(" {}", op))?;
                    }
                    support.to_css(dest)
                })
                .collect()
        }

        match self {
            SupportsCondition::Not(support) => {
                dest.write_str("not ")?;
                support.to_css(dest)
            },
            SupportsCondition::Parenthesized(support) => {
                dest.write_char('(')?;
                support.to_css(dest)?;
                dest.write_char(')')
            },
            SupportsCondition::And(supports) => write_for_vector(supports, "and", dest),
            SupportsCondition::Or(supports) => write_for_vector(supports, "or", dest),
            SupportsCondition::Declaration(declaration) => declaration.to_css(dest),
            SupportsCondition::GeneralEnclosed(value) => value.to_css(dest),
        }
    }
}

#[derive(Clone, Debug)]
/// A possibly-invalid property declaration
pub struct Declaration(pub String);

impl Declaration {
    pub fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i>> {
        let pos = input.position();
        input.expect_ident()?;
        input.expect_colon()?;
        consume_any_value(input)?;
        Ok(Declaration(input.slice_from(pos).to_owned()))
    }
}

impl ToCss for Declaration {
    fn to_css<W>(&self, dest: &mut crate::css_writer::CssWriter<W>) -> core::fmt::Result
    where
        W: std::fmt::Write,
    {
        dest.write_str(&self.0)
    }
}
