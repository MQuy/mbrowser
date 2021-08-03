use std::fmt;

use cssparser::ToCss;

use super::select_impl::SelectorImpl;

/// A pseudo-element, both public and private.
///
/// NB: If you add to this list, be sure to update `each_simple_pseudo_element` too.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[repr(usize)]
pub enum PseudoElement {
    // Eager pseudos. Keep these first so that eager_index() works.
    After = 0,
    Before,
    Selection,
    // If/when :first-letter is added, update is_first_letter accordingly.

    // If/when :first-line is added, update is_first_line accordingly.

    // If/when ::first-letter, ::first-line, or ::placeholder are added, adjust
    // our property_restriction implementation to do property filtering for
    // them.  Also, make sure the UA sheet has the !important rules some of the
    // APPLIES_TO_PLACEHOLDER properties expect!

    // Non-eager pseudos.
    DetailsSummary,
    DetailsContent,
    ServoText,
    ServoInputText,
    ServoTableWrapper,
    ServoAnonymousTableWrapper,
    ServoAnonymousTable,
    ServoAnonymousTableRow,
    ServoAnonymousTableCell,
    ServoAnonymousBlock,
    ServoInlineBlockWrapper,
    ServoInlineAbsolute,
}

/// The count of all pseudo-elements.
pub const PSEUDO_COUNT: usize = PseudoElement::ServoInlineAbsolute as usize + 1;

impl ToCss for PseudoElement {
    fn to_css<W>(&self, dest: &mut W) -> fmt::Result
    where
        W: fmt::Write,
    {
        use self::PseudoElement::*;
        dest.write_str(match *self {
            After => "::after",
            Before => "::before",
            Selection => "::selection",
            DetailsSummary => "::-servo-details-summary",
            DetailsContent => "::-servo-details-content",
            ServoText => "::-servo-text",
            ServoInputText => "::-servo-input-text",
            ServoTableWrapper => "::-servo-table-wrapper",
            ServoAnonymousTableWrapper => "::-servo-anonymous-table-wrapper",
            ServoAnonymousTable => "::-servo-anonymous-table",
            ServoAnonymousTableRow => "::-servo-anonymous-table-row",
            ServoAnonymousTableCell => "::-servo-anonymous-table-cell",
            ServoAnonymousBlock => "::-servo-anonymous-block",
            ServoInlineBlockWrapper => "::-servo-inline-block-wrapper",
            ServoInlineAbsolute => "::-servo-inline-absolute",
        })
    }
}

impl ::selectors::parser::PseudoElement for PseudoElement {
    type Impl = SelectorImpl;
}

/// The number of eager pseudo-elements. Keep this in sync with cascade_type.
pub const EAGER_PSEUDO_COUNT: usize = 3;

impl PseudoElement {
    /// Whether this pseudo-element is enabled for all content.
    pub fn enabled_in_content(&self) -> bool {
        todo!()
    }

    /// Whether this pseudo is enabled explicitly in UA sheets.
    pub fn enabled_in_ua_sheets(&self) -> bool {
        todo!()
    }

    /// Whether this pseudo is enabled explicitly in chrome sheets.
    pub fn enabled_in_chrome(&self) -> bool {
        todo!()
    }
}

/// This enumeration determines if a pseudo-element is eagerly cascaded or not.
///
/// If you're implementing a public selector for `Servo` that the end-user might
/// customize, then you probably need to make it eager.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PseudoElementCascadeType {
    /// Eagerly cascaded pseudo-elements are "normal" pseudo-elements (i.e.
    /// `::before` and `::after`). They inherit styles normally as another
    /// selector would do, and they're computed as part of the cascade.
    Eager,
    /// Lazy pseudo-elements are affected by selector matching, but they're only
    /// computed when needed, and not before. They're useful for general
    /// pseudo-elements that are not very common.
    ///
    /// Note that in Servo lazy pseudo-elements are restricted to a subset of
    /// selectors, so you can't use it for public pseudo-elements. This is not
    /// the case with Gecko though.
    Lazy,
    /// Precomputed pseudo-elements skip the cascade process entirely, mostly as
    /// an optimisation since they are private pseudo-elements (like
    /// `::-servo-details-content`).
    ///
    /// This pseudo-elements are resolved on the fly using *only* global rules
    /// (rules of the form `*|*`), and applying them to the parent style.
    Precomputed,
}
