use cssparser::SourceLocation;
use html5ever::{Namespace, Prefix};

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
