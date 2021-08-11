use std::fmt::Write;

use cssparser::SourceLocation;

use super::css_rule::CssRule;
use crate::css_writer::ToCss;
use crate::media_queries::media_list::MediaList;

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

impl ToCss for MediaRule {
    fn to_css<W>(&self, dest: &mut crate::css_writer::CssWriter<W>) -> core::fmt::Result
    where
        W: std::fmt::Write,
    {
        dest.write_str("@media ")?;
        self.media_queries.to_css(dest)?;
        dest.write_str(" {\n")?;
        // TODO: log for rules
        dest.write_str("}")
    }
}
