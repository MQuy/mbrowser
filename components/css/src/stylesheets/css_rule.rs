use core::fmt;

use super::keyframe_rule::KeyframesRule;
use super::media_rule::MediaRule;
use super::namespace_rule::NamespaceRule;
use super::page_rule::PageRule;
use super::rule_parser::State;
use super::style_rule::StyleRule;
use super::support_rule::SupportsRule;
use crate::css_writer::ToCss;

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

impl ToCss for CssRule {
    fn to_css<W>(&self, dest: &mut crate::css_writer::CssWriter<W>) -> fmt::Result
    where
        W: std::fmt::Write,
    {
        match &self {
            CssRule::Namespace(namespace) => namespace.to_css(dest),
            CssRule::Style(_) => todo!(),
            CssRule::Media(media_rule) => media_rule.to_css(dest),
            CssRule::Keyframes(keyframes) => keyframes.to_css(dest),
            CssRule::Supports(supports) => supports.to_css(dest),
            CssRule::Page(page) => page.to_css(dest),
        }
    }
}
