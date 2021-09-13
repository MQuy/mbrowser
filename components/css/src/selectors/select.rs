use super::nonts_pseudo_class::NonTSPseudoClass;
use super::pseudo_element::PseudoElement;
use crate::values::{CSSString, Ident};
use crate::{LocalName, Namespace, Prefix};

#[derive(Clone, Debug, PartialEq)]
pub struct Selectors;

impl ::selectors::SelectorImpl for Selectors {
	type AttrValue = CSSString;
	type BorrowedLocalName = html5ever::LocalName;
	type BorrowedNamespaceUrl = html5ever::Namespace;
	type ExtraMatchingData = InvalidationMatchingData;
	type Identifier = Ident;
	type LocalName = LocalName;
	type NamespacePrefix = Prefix;
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
