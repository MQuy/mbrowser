use std::fmt::Write;

use common::url::BrowserUrl;
use cssparser::{CowRcStr, Parser, SourceLocation, Token};
use html5ever::Prefix;

use super::rule_parser::StyleParseErrorKind;
use crate::css_writer::{CssWriter, ToCss};
use crate::parser::ParseError;

/// A `@namespace` rule.
#[derive(Clone, Debug, PartialEq)]
#[allow(missing_docs)]
pub struct NamespaceRule {
    /// The namespace prefix, and `None` if it's the default Namespace
    pub prefix: Option<Prefix>,
    /// The actual namespace url.
    pub value: NamespaceValue,
    /// The source location this rule was found at.
    pub source_location: SourceLocation,
}

impl NamespaceRule {
    /// https://drafts.csswg.org/css-namespaces/#syntax
    pub fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i>> {
        let prefix = input
            .try_parse(|i| i.expect_ident_cloned())
            .map(|s| Prefix::from(s.as_ref()))
            .ok();
        let value = NamespaceValue::parse(input)?;
        Ok(Self {
            prefix,
            value,
            source_location: input.current_source_location(),
        })
    }
}

impl ToCss for NamespaceRule {
    fn to_css<W>(&self, dest: &mut CssWriter<W>) -> core::fmt::Result
    where
        W: std::fmt::Write,
    {
        dest.write_str("@namespace ")?;
        if let Some(prefix) = &self.prefix {
            dest.write_fmt(format_args!("{} ", prefix))?;
        }
        dest.write_fmt(format_args!("{};", self.value.to_css_string()))?;
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum NamespaceValue {
    String(String),
    Url(BrowserUrl),
}

impl NamespaceValue {
    pub fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i>> {
        let location = input.current_source_location();
        match input.next()? {
            Token::UnquotedUrl(ref value) => NamespaceValue::parse_string_to_url(value, &location),
            Token::QuotedString(ref value) => Ok(NamespaceValue::String(value.to_string())),
            Token::Function(ref name) if name.eq_ignore_ascii_case("url") => input
                .parse_nested_block(|input| {
                    let location = input.current_source_location();
                    let url = input.expect_string()?;
                    NamespaceValue::parse_string_to_url(url, &location)
                }),
            t => {
                return Err(location.new_custom_error(
                    StyleParseErrorKind::UnexpectedTokenWithinNamespace(t.clone()),
                ))
            },
        }
    }

    fn parse_string_to_url<'i>(
        value: &CowRcStr<'i>,
        location: &SourceLocation,
    ) -> Result<Self, ParseError<'i>> {
        let browser_url = BrowserUrl::parse(value).map_err(|_err| {
            location.new_custom_error(StyleParseErrorKind::UnexpectedValue(value.clone()))
        })?;
        Ok(NamespaceValue::Url(browser_url))
    }
}

impl ToCss for NamespaceValue {
    fn to_css<W>(&self, dest: &mut CssWriter<W>) -> core::fmt::Result
    where
        W: Write,
    {
        match self {
            NamespaceValue::String(value) => dest.write_fmt(format_args!("\"{}\"", value)),
            NamespaceValue::Url(url) => dest.write_fmt(format_args!("url({})", url.as_str())),
        }
    }
}
