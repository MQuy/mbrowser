use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

use encoding_rs::Encoding;
use html5ever::{ns, tree_builder::QuirksMode, LocalName, Namespace};
use mime::Mime;

use crate::{
    customelementregistry::CustomElementDefinition,
    htmlbaseelement::HTMLBaseElement,
    inheritance::{Castable, DerivedFrom},
    node::Node,
    url::BrowserUrl,
};
use html5ever::namespace_url;

#[derive(Clone)]
pub struct Document {
    node: Node,
    content_type: Mime,
    url: RefCell<BrowserUrl>,
    encoding: &'static Encoding,
    quirk_mode: Cell<QuirksMode>,
    base_element: Rc<HTMLBaseElement>,
}

impl Document {
    pub fn set_quirks_mode(&self, mode: QuirksMode) {
        self.quirk_mode.set(mode);
    }

    pub fn lookup_custom_element_definition(
        &self,
        namespace: &Namespace,
        local_name: &LocalName,
        is: Option<&LocalName>,
    ) -> Option<Rc<CustomElementDefinition>> {
        if ns!(html) != *namespace {
            return None;
        }
        todo!();
    }

    /// Returns the first `base` element in the DOM that has an `href` attribute.
    pub fn base_element(&self) -> Option<Rc<HTMLBaseElement>> {
        Some(self.base_element.clone())
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
