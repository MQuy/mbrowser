use std::collections::HashMap;
use std::rc::Rc;

use cssparser::{Parser, ParserInput, RuleListParser};
use html5ever::{Namespace, Prefix};

use crate::media_queries::media_list::MediaList;

use super::css_rule::{CssRule, CssRuleType};
use super::origin::Origin;
use super::rule_parser::{State, TopLevelRuleParser};

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
        line_number_offset: u32,
    ) -> (Vec<CssRule>, Option<String>) {
        let mut rules = Vec::new();
        let mut input = ParserInput::new_with_line_number_offset(css, line_number_offset);
        let mut input = Parser::new(&mut input);

        let context = ParserContext::new(origin, None, quirks_mode);

        let rule_parser = TopLevelRuleParser {
            context,
            state: State::Start,
            namespaces: namespaces,
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
                    Ok((rule_start, rule)) => {
                        // Use a fallible push here, and if it fails, just fall
                        // out of the loop.  This will cause the page to be
                        // shown incorrectly, but it's better than OOMing.
                        rules.push(rule)
                    },
                    Err((error, slice)) => {
                        todo!()
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
        quirks_mode: QuirksMode,
        line_number_offset: u32,
    ) -> Self {
        let mut namespaces = Namespaces::default();
        let (rules, source_url) = Stylesheet::parse_rules(
            css,
            origin,
            quirks_mode,
            &mut namespaces,
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

pub struct ParserContext<'a> {
    /// The `Origin` of the stylesheet, whether it's a user, author or
    /// user-agent stylesheet.
    pub stylesheet_origin: Origin,
    /// The current rule type, if any.
    pub rule_type: Option<CssRuleType>,
    /// The quirks mode of this stylesheet.
    pub quirks_mode: QuirksMode,
    /// The currently active namespaces.
    pub namespaces: Option<&'a Namespaces>,
}

impl<'a> ParserContext<'a> {
    pub fn new(
        stylesheet_origin: Origin,
        rule_type: Option<CssRuleType>,
        quirks_mode: QuirksMode,
    ) -> Self {
        Self {
            stylesheet_origin,
            rule_type,
            quirks_mode,
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
}

#[allow(missing_docs)]
pub enum RulesMutateError {
    Syntax,
    IndexSize,
    HierarchyRequest,
    InvalidState,
}
