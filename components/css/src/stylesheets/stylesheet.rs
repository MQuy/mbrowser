use std::collections::HashMap;
use std::fmt::Display;
use std::ops::Add;
use std::rc::Rc;

use cssparser::{Parser, ParserInput, RuleListParser, SourceLocation};
use html5ever::{Namespace, Prefix};

use super::css_rule::{CssRule, CssRuleType};
use super::origin::Origin;
use super::rule_parser::{State, TopLevelRuleParser};
use crate::css_writer::ToCss;
use crate::error_reporting::{ContextualParseError, ParseErrorReporter};
use crate::media_queries::media_list::MediaList;

/// Which quirks mode is this document in.
///
/// See: https://quirks.spec.whatwg.org/
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum QuirksMode {
    /// Quirks mode.
    Quirks,
    /// Limited quirks mode.
    LimitedQuirks,
    /// No quirks mode.
    NoQuirks,
}

/// A set of namespaces applying to a given stylesheet.
///
/// The namespace id is used in gecko
#[derive(Clone, Debug, Default)]
#[allow(missing_docs)]
pub struct Namespaces {
    pub default: Option<Namespace>,
    pub prefixes: HashMap<Prefix, Namespace>,
}

pub struct Stylesheet {
    pub rules: Vec<CssRule>,
    pub origin: Origin,
    pub quirks_mode: QuirksMode,
    pub namespaces: Namespaces,
    pub source_url: Option<String>,
    pub media: Rc<MediaList>,
    pub disabled: bool,
}

impl Stylesheet {
    pub fn parse_rules(
        css: &str,
        origin: Origin,
        quirks_mode: QuirksMode,
        namespaces: &mut Namespaces,
        error_reporter: Option<&dyn ParseErrorReporter>,
        line_number_offset: u32,
    ) -> (Vec<CssRule>, Option<String>) {
        let mut rules = Vec::new();
        let mut input = ParserInput::new_with_line_number_offset(css, line_number_offset);
        let mut input = Parser::new(&mut input);

        let context = ParserContext::new(origin, None, quirks_mode, error_reporter);

        let rule_parser = TopLevelRuleParser {
            context,
            namespaces,
            state: State::Start,
            dom_error: None,
            insert_rule_context: None,
        };

        {
            let mut iter = RuleListParser::new_for_stylesheet(&mut input, rule_parser);
            loop {
                let result = match iter.next() {
                    Some(result) => result,
                    None => break,
                };
                match result {
                    Ok((_rule_start, rule)) => {
                        // Use a fallible push here, and if it fails, just fall
                        // out of the loop.  This will cause the page to be
                        // shown incorrectly, but it's better than OOMing.
                        rules.push(rule)
                    },
                    Err((error, slice)) => {
                        let location = error.location;
                        let error = ContextualParseError::InvalidRule(slice, error);
                        iter.parser.context.log_css_error(location, error);
                    },
                }
            }
        }

        let source_url = input.current_source_url().map(String::from);
        (rules, source_url)
    }

    pub fn from_str(
        css: &str,
        origin: Origin,
        media: Rc<MediaList>,
        error_reporter: Option<&dyn ParseErrorReporter>,
        quirks_mode: QuirksMode,
        line_number_offset: u32,
    ) -> Self {
        let mut namespaces = Namespaces::default();
        let (rules, source_url) = Stylesheet::parse_rules(
            css,
            origin,
            quirks_mode,
            &mut namespaces,
            error_reporter,
            line_number_offset,
        );

        Self {
            rules,
            origin,
            quirks_mode,
            source_url,
            namespaces,
            media,
            disabled: false,
        }
    }
}

impl Display for Stylesheet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.rules
            .iter()
            .map(|rule| f.write_str(&rule.to_css_string().add("\n")))
            .collect()
    }
}

pub struct ParserContext<'a> {
    /// The `Origin` of the stylesheet, whether it's a user, author or
    /// user-agent stylesheet.
    pub stylesheet_origin: Origin,
    /// The current rule type, if any.
    pub rule_type: Option<CssRuleType>,
    /// The quirks mode of this stylesheet.
    pub quirks_mode: QuirksMode,
    /// The active error reporter, or none if error reporting is disabled.
    pub error_reporter: Option<&'a dyn ParseErrorReporter>,
    /// The currently active namespaces.
    pub namespaces: Option<&'a Namespaces>,
}

impl<'a> ParserContext<'a> {
    pub fn new(
        stylesheet_origin: Origin,
        rule_type: Option<CssRuleType>,
        quirks_mode: QuirksMode,
        error_reporter: Option<&'a dyn ParseErrorReporter>,
    ) -> Self {
        Self {
            stylesheet_origin,
            rule_type,
            quirks_mode,
            error_reporter,
            namespaces: None,
        }
    }

    /// Create a parser context based on a previous context, but with a modified
    /// rule type.
    #[inline]
    pub fn new_with_rule_type(
        context: &'a ParserContext,
        rule_type: CssRuleType,
        namespaces: &'a Namespaces,
    ) -> ParserContext<'a> {
        Self {
            stylesheet_origin: context.stylesheet_origin,
            rule_type: Some(rule_type),
            quirks_mode: context.quirks_mode,
            namespaces: Some(namespaces),
            error_reporter: context.error_reporter,
        }
    }

    /// Get the rule type, which assumes that one is available.
    pub fn rule_type(&self) -> CssRuleType {
        self.rule_type
            .expect("Rule type expected, but none was found.")
    }

    /// Returns whether CSS error reporting is enabled.
    #[inline]
    pub fn error_reporting_enabled(&self) -> bool {
        false
    }

    /// Record a CSS parse error with this context’s error reporting.
    pub fn log_css_error(&self, location: SourceLocation, error: ContextualParseError) {
        let error_reporter = match self.error_reporter {
            Some(r) => r,
            None => return,
        };

        error_reporter.report_error(location, error)
    }
}

#[allow(missing_docs)]
pub enum RulesMutateError {
    Syntax,
    IndexSize,
    HierarchyRequest,
    InvalidState,
}
