use std::fmt::Write;

use cssparser::SourceLocation;

use crate::css_writer::ToCss;
use crate::properties::declaration_block::PropertyDeclarationBlock;

/// A [`@page`][page] rule.
///
/// This implements only a limited subset of the CSS
/// 2.2 syntax.
///
/// In this subset, [page selectors][page-selectors] are not implemented.
///
/// [page]: https://drafts.csswg.org/css2/page.html#page-box
/// [page-selectors]: https://drafts.csswg.org/css2/page.html#page-selectors
#[derive(Clone)]
pub struct PageRule {
    /// The declaration block this page rule contains.
    pub block: PropertyDeclarationBlock,
    /// The source position this rule was found at.
    pub source_location: SourceLocation,
}

impl ToCss for PageRule {
    fn to_css<W>(&self, dest: &mut crate::css_writer::CssWriter<W>) -> core::fmt::Result
    where
        W: std::fmt::Write,
    {
        dest.write_str(&self.block.to_css_string())
    }
}
