use html5ever::Namespace;

use super::nonts_pseudo_class::NonTSPseudoClass;
use super::pseudo_element::PseudoElement;

use crate::element_state::DocumentState;
use crate::values::CustomIdent;

#[derive(Clone, Debug, PartialEq)]
pub struct SelectorImpl;

impl ::selectors::SelectorImpl for SelectorImpl {
    type ExtraMatchingData = InvalidationMatchingData;
    type AttrValue = CustomIdent;
    type Identifier = CustomIdent;
    type LocalName = CustomIdent;
    type NamespacePrefix = CustomIdent;
    type NamespaceUrl = Namespace;
    type BorrowedLocalName = CustomIdent;
    type BorrowedNamespaceUrl = Namespace;

    type PseudoElement = PseudoElement;
    type NonTSPseudoClass = NonTSPseudoClass;
}

/// A struct holding the members necessary to invalidate document state
/// selectors.
pub struct InvalidationMatchingData {
    /// The document state that has changed, which makes it always match.
    pub document_state: DocumentState,
}

impl Default for InvalidationMatchingData {
    #[inline(always)]
    fn default() -> Self {
        Self {
            document_state: DocumentState::empty(),
        }
    }
}
