use common::not_supported;
use cssparser::{
    match_ignore_ascii_case, AtRuleParser, AtRuleType, BasicParseError, BasicParseErrorKind,
    CowRcStr, Parser, ParserState, QualifiedRuleParser, RuleListParser, SourcePosition, Token,
    _cssparser_internal_to_lowercase,
};
use html5ever::{Namespace, Prefix};
use selectors::parser::SelectorParseErrorKind;
use selectors::SelectorList;

use super::css_rule::{CssRule, CssRuleType};
use super::keyframe_rule::KeyframesRule;
use super::media_rule::MediaRule;
use super::namespace_rule::NamespaceRule;
use super::style_rule::StyleRule;
use super::stylesheet::{Namespaces, ParserContext};
use super::support_rule::{SupportsCondition, SupportsRule};
use crate::media_queries::media_list::MediaList;
use crate::parser::ParseError;
use crate::properties::declaration_block::parse_property_declaration_list;
use crate::selectors::select_impl::SelectorImpl;
use crate::selectors::selector_parser::SelectorParser;
use crate::str::starts_with_ignore_ascii_case;
use crate::stylesheets::keyframe_rule::parse_keyframe_list;
use crate::stylesheets::page_rule::PageRule;
use crate::stylesheets::stylesheet::RulesMutateError;
use crate::values::animation::KeyframesName;
use crate::values::url::CssUrl;

#[derive(Clone, Debug)]
/// Vendor prefix.
pub enum VendorPrefix {
    /// -moz prefix.
    Moz,
    /// -webkit prefix.
    WebKit,
}

/// A rule prelude for at-rule with block.
pub enum AtRuleBlockPrelude {
    /// A @media rule prelude, with its media queries.
    Media(MediaList),
    /// An @supports rule, with its conditional
    Supports(SupportsCondition),
    /// A @viewport rule prelude.
    Viewport,
    /// A @keyframes rule, with its animation name and vendor prefix if exists.
    Keyframes(KeyframesName, Option<VendorPrefix>),
    /// A @page rule prelude.
    Page,
}

/// A rule prelude for at-rule without block.
pub enum AtRuleNonBlockPrelude {
    /// A @import rule prelude.
    Import(CssUrl, MediaList),
    /// A @namespace rule prelude.
    Namespace(Option<Prefix>, Namespace),
}

/// The current state of the parser.
#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
pub enum State {
    /// We haven't started parsing rules.
    Start = 1,
    /// We're parsing `@import` rules.
    Imports = 2,
    /// We're parsing `@namespace` rules.
    Namespaces = 3,
    /// We're parsing the main body of the stylesheet.
    Body = 4,
}

/// The information we need particularly to do CSSOM insertRule stuff.
pub struct InsertRuleContext<'a> {
    /// The rule list we're about to insert into.
    pub rule_list: &'a [CssRule],
    /// The index we're about to get inserted at.
    pub index: usize,
}

pub struct TopLevelRuleParser<'a> {
    pub context: ParserContext<'a>,
    pub state: State,
    pub dom_error: Option<RulesMutateError>,
    pub namespaces: &'a mut Namespaces,
    pub insert_rule_context: Option<InsertRuleContext<'a>>,
}

impl<'b> TopLevelRuleParser<'b> {
    fn nested<'a: 'b>(&'a self) -> NestedRuleParser<'a, 'b> {
        NestedRuleParser {
            context: &self.context,
            namespaces: &self.namespaces,
        }
    }

    /// Returns the current state of the parser.
    pub fn state(&self) -> State {
        self.state
    }

    /// Checks whether we can parse a rule that would transition us to
    /// `new_state`.
    ///
    /// This is usually a simple branch, but we may need more bookkeeping if
    /// doing `insertRule` from CSSOM.
    fn check_state(&mut self, new_state: State) -> bool {
        if self.state > new_state {
            self.dom_error = Some(RulesMutateError::HierarchyRequest);
            return false;
        }

        let ctx = match self.insert_rule_context {
            Some(ref ctx) => ctx,
            None => return true,
        };

        let next_rule_state = match ctx.rule_list.get(ctx.index) {
            None => return true,
            Some(rule) => rule.rule_state(),
        };

        if new_state > next_rule_state {
            self.dom_error = Some(RulesMutateError::HierarchyRequest);
            return false;
        }

        // If there's anything that isn't a namespace rule (or import rule, but
        // we checked that already at the beginning), reject with a
        // StateError.
        if new_state == State::Namespaces
            && ctx.rule_list[ctx.index..]
                .iter()
                .any(|r| !matches!(*r, CssRule::Namespace(..)))
        {
            self.dom_error = Some(RulesMutateError::InvalidState);
            return false;
        }

        true
    }
}

impl<'a, 'i> AtRuleParser<'i> for TopLevelRuleParser<'a> {
    type AtRule = (SourcePosition, CssRule);
    type Error = StyleParseErrorKind<'i>;
    type PreludeBlock = AtRuleBlockPrelude;
    type PreludeNoBlock = AtRuleNonBlockPrelude;

    fn parse_prelude<'t>(
        &mut self,
        name: CowRcStr<'i>,
        input: &mut Parser<'i, 't>,
    ) -> Result<AtRuleType<AtRuleNonBlockPrelude, AtRuleBlockPrelude>, ParseError<'i>> {
        match_ignore_ascii_case! { &*name,
            "namespace" => {
                if !self.check_state(State::Namespaces) {
                    return Err(input.new_custom_error(StyleParseErrorKind::UnexpectedNamespaceRule))
                }

                let namespace = NamespaceRule::parse(input)?;
                let prelude = AtRuleNonBlockPrelude::Namespace(namespace.prefix, namespace.url);
                return Ok(AtRuleType::WithoutBlock(prelude));
            },
            _ => {}
        }

        AtRuleParser::parse_prelude(&mut self.nested(), name, input)
    }

    #[inline]
    fn parse_block<'t>(
        &mut self,
        prelude: AtRuleBlockPrelude,
        start: &ParserState,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::AtRule, ParseError<'i>> {
        let rule = AtRuleParser::parse_block(&mut self.nested(), prelude, start, input)?;
        self.state = State::Body;
        Ok((start.position(), rule))
    }

    #[inline]
    fn rule_without_block(
        &mut self,
        prelude: AtRuleNonBlockPrelude,
        start: &ParserState,
    ) -> Self::AtRule {
        let rule = match prelude {
            AtRuleNonBlockPrelude::Namespace(prefix, url) => {
                let prefix = if let Some(prefix) = prefix {
                    self.namespaces.prefixes.insert(prefix.clone(), url.clone());
                    Some(prefix)
                } else {
                    self.namespaces.default = Some(url.clone());
                    None
                };

                self.state = State::Namespaces;
                CssRule::Namespace(NamespaceRule {
                    prefix,
                    url,
                    source_location: start.source_location(),
                })
            },
            _ => not_supported!(),
        };

        (start.position(), rule)
    }
}

impl<'a, 'i> QualifiedRuleParser<'i> for TopLevelRuleParser<'a> {
    type Error = StyleParseErrorKind<'i>;
    type Prelude = SelectorList<SelectorImpl>;
    type QualifiedRule = (SourcePosition, CssRule);

    #[inline]
    fn parse_prelude<'t>(
        &mut self,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::Prelude, ParseError<'i>> {
        QualifiedRuleParser::parse_prelude(&mut self.nested(), input)
    }

    #[inline]
    fn parse_block<'t>(
        &mut self,
        prelude: Self::Prelude,
        start: &ParserState,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::QualifiedRule, ParseError<'i>> {
        let rule = QualifiedRuleParser::parse_block(&mut self.nested(), prelude, start, input)?;
        Ok((start.position(), rule))
    }
}

#[derive(Clone)] // shallow, relatively cheap .clone
struct NestedRuleParser<'a, 'b: 'a> {
    context: &'a ParserContext<'b>,
    namespaces: &'a Namespaces,
}

impl<'a, 'b> NestedRuleParser<'a, 'b> {
    fn parse_nested_rules(&mut self, input: &mut Parser, rule_type: CssRuleType) -> Vec<CssRule> {
        let context = ParserContext::new_with_rule_type(self.context, rule_type, self.namespaces);

        let nested_parser = NestedRuleParser {
            context: &context,
            namespaces: self.namespaces,
        };

        let mut iter = RuleListParser::new_for_nested_rule(input, nested_parser);
        let mut rules = Vec::new();
        while let Some(result) = iter.next() {
            match result {
                Ok(rule) => rules.push(rule),
                Err((error, slice)) => {
                    todo!()
                },
            }
        }
        rules
    }
}

impl<'a, 'b, 'i> AtRuleParser<'i> for NestedRuleParser<'a, 'b> {
    type AtRule = CssRule;
    type Error = StyleParseErrorKind<'i>;
    type PreludeBlock = AtRuleBlockPrelude;
    type PreludeNoBlock = AtRuleNonBlockPrelude;

    fn parse_prelude<'t>(
        &mut self,
        name: CowRcStr<'i>,
        input: &mut Parser<'i, 't>,
    ) -> Result<AtRuleType<AtRuleNonBlockPrelude, AtRuleBlockPrelude>, ParseError<'i>> {
        match_ignore_ascii_case! { &*name,
            "media" => {
                let media_queries = MediaList::parse(self.context, input);
                Ok(AtRuleType::WithBlock(AtRuleBlockPrelude::Media(media_queries)))
            },
            "supports" => {
                let cond = SupportsCondition::parse(input)?;
                Ok(AtRuleType::WithBlock(AtRuleBlockPrelude::Supports(cond)))
            },
            "keyframes" | "-webkit-keyframes" | "-moz-keyframes" => {
                let prefix = if starts_with_ignore_ascii_case(&*name, "-webkit-") {
                    Some(VendorPrefix::WebKit)
                } else if starts_with_ignore_ascii_case(&*name, "-moz-") {
                    Some(VendorPrefix::Moz)
                } else {
                    None
                };
                let name = KeyframesName::parse(self.context, input)?;
                Ok(AtRuleType::WithBlock(AtRuleBlockPrelude::Keyframes(name, prefix)))
            },
            "page" => {
                Ok(AtRuleType::WithBlock(AtRuleBlockPrelude::Page))
            },
            _ => Err(input.new_custom_error(StyleParseErrorKind::UnsupportedAtRule(name.clone())))
        }
    }

    fn parse_block<'t>(
        &mut self,
        prelude: AtRuleBlockPrelude,
        start: &ParserState,
        input: &mut Parser<'i, 't>,
    ) -> Result<CssRule, ParseError<'i>> {
        match prelude {
            AtRuleBlockPrelude::Media(media_queries) => Ok(CssRule::Media(MediaRule {
                media_queries,
                rules: self.parse_nested_rules(input, CssRuleType::Media),
                source_location: start.source_location(),
            })),
            AtRuleBlockPrelude::Supports(condition) => {
                let eval_context = ParserContext::new_with_rule_type(
                    self.context,
                    CssRuleType::Style,
                    self.namespaces,
                );

                let enabled = condition.eval(&eval_context, self.namespaces);
                Ok(CssRule::Supports(SupportsRule {
                    condition,
                    rules: self.parse_nested_rules(input, CssRuleType::Supports),
                    enabled,
                    source_location: start.source_location(),
                }))
            },
            AtRuleBlockPrelude::Keyframes(name, vendor_prefix) => {
                let context = ParserContext::new_with_rule_type(
                    self.context,
                    CssRuleType::Keyframes,
                    self.namespaces,
                );

                Ok(CssRule::Keyframes(KeyframesRule {
                    name,
                    keyframes: parse_keyframe_list(&context, input),
                    vendor_prefix,
                    source_location: start.source_location(),
                }))
            },
            AtRuleBlockPrelude::Page => {
                let context = ParserContext::new_with_rule_type(
                    self.context,
                    CssRuleType::Page,
                    self.namespaces,
                );

                let declarations = parse_property_declaration_list(&context, input, None);
                Ok(CssRule::Page(PageRule {
                    block: declarations,
                    source_location: start.source_location(),
                }))
            },
            _ => not_supported!(),
        }
    }
}

impl<'a, 'b, 'i> QualifiedRuleParser<'i> for NestedRuleParser<'a, 'b> {
    type Error = StyleParseErrorKind<'i>;
    type Prelude = SelectorList<SelectorImpl>;
    type QualifiedRule = CssRule;

    fn parse_prelude<'t>(
        &mut self,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::Prelude, ParseError<'i>> {
        let selector_parser = SelectorParser {
            stylesheet_origin: self.context.stylesheet_origin,
            namespaces: self.namespaces,
        };
        SelectorList::parse(&selector_parser, input)
    }

    fn parse_block<'t>(
        &mut self,
        selectors: Self::Prelude,
        start: &ParserState,
        input: &mut Parser<'i, 't>,
    ) -> Result<CssRule, ParseError<'i>> {
        let context =
            ParserContext::new_with_rule_type(self.context, CssRuleType::Style, self.namespaces);

        let declarations = parse_property_declaration_list(&context, input, Some(&selectors));
        Ok(CssRule::Style(StyleRule {
            selectors,
            block: declarations,
            source_location: start.source_location(),
        }))
    }
}

#[derive(Clone, Debug, PartialEq)]
/// Errors that can be encountered while parsing CSS values.
pub enum StyleParseErrorKind<'i> {
    /// A bad URL token in a DVB.
    BadUrlInDeclarationValueBlock(CowRcStr<'i>),
    /// A bad string token in a DVB.
    BadStringInDeclarationValueBlock(CowRcStr<'i>),
    /// Unexpected closing parenthesis in a DVB.
    UnbalancedCloseParenthesisInDeclarationValueBlock,
    /// Unexpected closing bracket in a DVB.
    UnbalancedCloseSquareBracketInDeclarationValueBlock,
    /// Unexpected closing curly bracket in a DVB.
    UnbalancedCloseCurlyBracketInDeclarationValueBlock,
    /// A property declaration value had input remaining after successfully parsing.
    PropertyDeclarationValueNotExhausted,
    /// An unexpected dimension token was encountered.
    UnexpectedDimension(CowRcStr<'i>),
    /// Unexpect token in media query.
    MediaQueryExpectedToken,
    /// Missing or invalid media feature name.
    MediaQueryExpectedFeatureName(CowRcStr<'i>),
    /// Missing or invalid media feature value.
    MediaQueryExpectedFeatureValue,
    /// A media feature range operator was not expected.
    MediaQueryUnexpectedOperator,
    /// min- or max- properties must have a value.
    RangedExpressionWithNoValue,
    /// A function was encountered that was not expected.
    UnexpectedFunction(CowRcStr<'i>),
    /// @namespace must be before any rule but @charset and @import
    UnexpectedNamespaceRule,
    /// @import must be before any rule but @charset
    UnexpectedImportRule,
    /// @import rules are disallowed in the parser.
    DisallowedImportRule,
    /// Unexpected @charset rule encountered.
    UnexpectedCharsetRule,
    /// Unsupported @ rule
    UnsupportedAtRule(CowRcStr<'i>),
    /// A placeholder for many sources of errors that require more specific variants.
    UnspecifiedError,
    /// An unexpected token was found within a namespace rule.
    UnexpectedTokenWithinNamespace(Token<'i>),
    /// An error was encountered while parsing a property value.
    ValueError(ValueParseErrorKind<'i>),
    /// An error was encountered while parsing a selector
    SelectorError(SelectorParseErrorKind<'i>),
    /// The property declaration was for an unknown property.
    UnknownProperty(CowRcStr<'i>),
    /// The property declaration was for a disabled experimental property.
    ExperimentalProperty,
    /// The property declaration contained an invalid color value.
    InvalidColor(CowRcStr<'i>, Token<'i>),
    /// The property declaration contained an invalid filter value.
    InvalidFilter(CowRcStr<'i>, Token<'i>),
    /// The property declaration contained an invalid value.
    OtherInvalidValue(CowRcStr<'i>),
    /// The declaration contained an animation property, and we were parsing
    /// this as a keyframe block (so that property should be ignored).
    ///
    /// See: https://drafts.csswg.org/css-animations/#keyframes
    AnimationPropertyInKeyframeBlock,
    /// The property is not allowed within a page rule.
    NotAllowedInPageRule,
}

/// Specific errors that can be encountered while parsing property values.
#[derive(Clone, Debug, PartialEq)]
pub enum ValueParseErrorKind<'i> {
    /// An invalid token was encountered while parsing a color value.
    InvalidColor(Token<'i>),
    /// An invalid filter value was encountered.
    InvalidFilter(Token<'i>),
}

impl<'i> From<ValueParseErrorKind<'i>> for StyleParseErrorKind<'i> {
    fn from(this: ValueParseErrorKind<'i>) -> Self {
        StyleParseErrorKind::ValueError(this)
    }
}

impl<'i> From<SelectorParseErrorKind<'i>> for StyleParseErrorKind<'i> {
    fn from(this: SelectorParseErrorKind<'i>) -> Self {
        StyleParseErrorKind::SelectorError(this)
    }
}

impl<'i> StyleParseErrorKind<'i> {
    /// Create an InvalidValue parse error
    pub fn new_invalid<S>(name: S, value_error: ParseError<'i>) -> ParseError<'i>
    where
        S: Into<CowRcStr<'i>>,
    {
        let name = name.into();
        let variant = match value_error.kind {
            cssparser::ParseErrorKind::Custom(StyleParseErrorKind::ValueError(e)) => match e {
                ValueParseErrorKind::InvalidColor(token) => {
                    StyleParseErrorKind::InvalidColor(name, token)
                },
                ValueParseErrorKind::InvalidFilter(token) => {
                    StyleParseErrorKind::InvalidFilter(name, token)
                },
            },
            _ => StyleParseErrorKind::OtherInvalidValue(name),
        };
        cssparser::ParseError {
            kind: cssparser::ParseErrorKind::Custom(variant),
            location: value_error.location,
        }
    }
}
