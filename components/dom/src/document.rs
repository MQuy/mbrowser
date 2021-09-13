use std::cell::{Cell, RefCell};
use std::rc::Rc;

use common::url::BrowserUrl;
use encoding_rs::{Encoding, UTF_8};
use html5ever::tree_builder::QuirksMode;
use mime::Mime;

use crate::htmlbaseelement::HTMLBaseElement;
use crate::inheritance::{Castable, DerivedFrom};
use crate::node::Node;

#[derive(Clone, Debug)]
#[repr(C)]
pub struct Document {
	node: Node,
	content_type: Mime,
	url: RefCell<BrowserUrl>,
	encoding: &'static Encoding,
	quirks_mode: Cell<QuirksMode>,
	base_element: RefCell<Option<Rc<HTMLBaseElement>>>,
}

impl Document {
	pub fn new(url: Option<BrowserUrl>) -> Self {
		let url = url.unwrap_or_else(|| BrowserUrl::parse("about:blank").unwrap());
		Self {
			node: Node::new(crate::nodetype::NodeTypeId::Document, None),
			content_type: mime::TEXT_HTML,
			encoding: UTF_8,
			quirks_mode: Cell::new(QuirksMode::NoQuirks),
			base_element: RefCell::new(None),
			url: RefCell::new(url),
		}
	}

	pub fn set_quirks_mode(&self, mode: QuirksMode) {
		self.quirks_mode.set(mode);
	}

	/// Returns the first `base` element in the DOM that has an `href` attribute.
	pub fn base_element(&self) -> Option<Rc<HTMLBaseElement>> {
		self.base_element.borrow().clone()
	}

	// https://html.spec.whatwg.org/multipage/#document-base-url
	pub fn base_url(&self) -> BrowserUrl {
		match self.base_element() {
			// Step 1.
			None => self.fallback_base_url(),
			// Step 2.
			Some(base) => base.frozen_base_url(),
		}
	}

	// https://dom.spec.whatwg.org/#concept-document-url
	pub fn url(&self) -> BrowserUrl {
		self.url.borrow().clone()
	}

	// https://html.spec.whatwg.org/multipage/#fallback-base-url
	pub fn fallback_base_url(&self) -> BrowserUrl {
		self.url()
	}
}

impl Castable for Document {}
impl DerivedFrom<Node> for Document {}
