use cssparser::SourceLocation;

use crate::media_queries::media_list::MediaList;

use super::css_rule::CssRule;

/// An [`@media`][media] urle.
///
/// [media]: https://drafts.csswg.org/css-conditional/#at-ruledef-media
#[derive(Clone)]
pub struct MediaRule {
    /// The list of media queries used by this media rule.
    pub media_queries: MediaList,
    /// The nested rules to this media rule.
    pub rules: Vec<CssRule>,
    /// The source position where this media rule was found.
    pub source_location: SourceLocation,
}
