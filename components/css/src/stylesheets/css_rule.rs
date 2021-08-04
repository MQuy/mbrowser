use cssparser::{parse_one_rule, Parser, ParserInput};

use super::keyframe_rule::KeyframesRule;
use super::media_rule::MediaRule;
use super::namespace_rule::NamespaceRule;
use super::page_rule::PageRule;
use super::rule_parser::{InsertRuleContext, State, TopLevelRuleParser};
use super::style_rule::StyleRule;
use super::stylesheet::{ParserContext, RulesMutateError, Stylesheet};
use super::support_rule::SupportsRule;
use crate::error_reporting::ParseErrorReporter;

/// A CSS rule.
/// https://drafts.csswg.org/cssom/#concept-css-rule-type
#[derive(Clone)]
pub enum CssRule {
    Namespace(NamespaceRule),
    Style(StyleRule),
    Media(MediaRule),
    Keyframes(KeyframesRule),
    Supports(SupportsRule),
    Page(PageRule),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CssRuleType {
    // https://drafts.csswg.org/cssom/#the-cssrule-interface
    Style = 1,
    Media = 4,
    Page = 6,
    // https://drafts.csswg.org/css-animations-1/#interface-cssrule-idl
    Keyframes = 7,
    Keyframe = 8,
    Namespace = 10,
    // https://drafts.csswg.org/css-conditional-3/#extentions-to-cssrule-interface
    Supports = 12,
}

impl CssRule {
    pub fn rule_state(&self) -> State {
        match *self {
            CssRule::Namespace(..) => State::Namespaces,
            _ => State::Body,
        }
    }
}
