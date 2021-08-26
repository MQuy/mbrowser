use html5ever::Namespace;

use super::nonts_pseudo_class::NonTSPseudoClass;
use super::pseudo_element::PseudoElement;
use crate::values::CustomIdent;

#[derive(Clone, Debug, PartialEq)]
pub struct SelectorImpl;

impl ::selectors::SelectorImpl for SelectorImpl {
	type AttrValue = CustomIdent;
	type BorrowedLocalName = CustomIdent;
	type BorrowedNamespaceUrl = Namespace;
	type ExtraMatchingData = InvalidationMatchingData;
	type Identifier = CustomIdent;
	type LocalName = CustomIdent;
	type NamespacePrefix = CustomIdent;
	type NamespaceUrl = Namespace;
	type NonTSPseudoClass = NonTSPseudoClass;
	type PseudoElement = PseudoElement;
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

bitflags! {
	/// Event-based document states.
	///
	/// NB: Is important for this to remain in sync with Gecko's
	/// dom/base/Document.h.
	pub struct DocumentState: u64 {
		/// RTL locale: specific to the XUL localedir attribute
		const NS_DOCUMENT_STATE_RTL_LOCALE = 1 << 0;
		/// Window activation status
		const NS_DOCUMENT_STATE_WINDOW_INACTIVE = 1 << 1;
	}
}
