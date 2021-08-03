use cssparser::{Parser, SourceLocation};

use crate::parser::ParseError;

use super::css_rule::CssRule;
use super::stylesheet::{Namespaces, ParserContext};

/// An [`@supports`][supports] rule.
///
/// [supports]: https://drafts.csswg.org/css-conditional-3/#at-supports
#[derive(Clone)]
pub struct SupportsRule {
    /// The parsed condition
    pub condition: SupportsCondition,
    /// Child rules
    pub rules: Vec<CssRule>,
    /// The result of evaluating the condition
    pub enabled: bool,
    /// The line and column of the rule's source code.
    pub source_location: SourceLocation,
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
    /// A `selector()` function.
    Selector(RawSelector),
    /// `(any tokens)` or `func(any tokens)`
    FutureSyntax(String),
}

impl SupportsCondition {
    /// Parse a condition
    ///
    /// <https://drafts.csswg.org/css-conditional/#supports_condition>
    pub fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i>> {
        todo!()
    }

    /// Evaluate a supports condition
    pub fn eval(&self, cx: &ParserContext, namespaces: &Namespaces) -> bool {
        match *self {
            SupportsCondition::Not(ref cond) => !cond.eval(cx, namespaces),
            SupportsCondition::Parenthesized(ref cond) => cond.eval(cx, namespaces),
            SupportsCondition::And(ref vec) => vec.iter().all(|c| c.eval(cx, namespaces)),
            SupportsCondition::Or(ref vec) => vec.iter().any(|c| c.eval(cx, namespaces)),
            SupportsCondition::Declaration(ref decl) => decl.eval(cx),
            SupportsCondition::Selector(ref selector) => selector.eval(cx, namespaces),
            SupportsCondition::FutureSyntax(_) => false,
        }
    }
}

#[derive(Clone, Debug)]
/// A possibly-invalid property declaration
pub struct Declaration(pub String);

/// <https://drafts.csswg.org/css-syntax-3/#typedef-any-value>
fn consume_any_value<'i, 't>(input: &mut Parser<'i, 't>) -> Result<(), ParseError<'i>> {
    input.expect_no_error_token().map_err(|err| err.into())
}

impl Declaration {
    /// Parse a declaration
    pub fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Declaration, ParseError<'i>> {
        let pos = input.position();
        input.expect_ident()?;
        input.expect_colon()?;
        consume_any_value(input)?;
        Ok(Declaration(input.slice_from(pos).to_owned()))
    }

    /// Determine if a declaration parses
    ///
    /// <https://drafts.csswg.org/css-conditional-3/#support-definition>
    pub fn eval(&self, context: &ParserContext) -> bool {
        todo!()
    }
}

#[derive(Clone, Debug)]
/// A possibly-invalid CSS selector.
pub struct RawSelector(pub String);

impl RawSelector {
    /// Tries to evaluate a `selector()` function.
    pub fn eval(&self, context: &ParserContext, namespaces: &Namespaces) -> bool {
        todo!()
    }
}
