use std::fmt::Write;

use cssparser::{BasicParseError, BasicParseErrorKind, Parser, SourceLocation};
use html5ever::{Namespace, Prefix};

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
    pub url: Namespace,
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
        let maybe_namespace = match input.expect_url_or_string() {
            Ok(url_or_string) => url_or_string,
            Err(BasicParseError {
                kind: BasicParseErrorKind::UnexpectedToken(t),
                location,
            }) => {
                return Err(location
                    .new_custom_error(StyleParseErrorKind::UnexpectedTokenWithinNamespace(t)))
            },
            Err(e) => return Err(e.into()),
        };
        let url = Namespace::from(maybe_namespace.as_ref());
        Ok(Self {
            prefix,
            url,
            source_location: input.current_source_location(),
        })
    }
}

impl ToCss for NamespaceRule {
    fn to_css<W>(&self, dest: &mut CssWriter<W>) -> core::fmt::Result
    where
        W: std::fmt::Write,
    {
        dest.write_str("@namespaces ")?;
        if let Some(prefix) = &self.prefix {
            dest.write_str(&std::format!("{} ", prefix))?;
        }
        dest.write_str(&self.url)?;
        Ok(())
    }
}
