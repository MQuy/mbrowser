use std::{rc::Weak, sync::Arc};

use html5ever::{local_name, ns, LocalName, Prefix};

use crate::{
    attr::AttrValue,
    document::Document,
    element::Element,
    htmlelement::HTMLElement,
    inheritance::{Castable, DerivedFrom},
    node::{document_from_node, Node},
    nodetype::{ElementTypeId, HTMLElementTypeId, NodeTypeId},
    url::BrowserUrl,
    virtualmethods::VirtualMethods,
};
use html5ever::namespace_url;

#[derive(Clone)]
pub struct HTMLBaseElement {
    htmlelement: HTMLElement,
}

impl Castable for HTMLBaseElement {}
impl DerivedFrom<Node> for HTMLBaseElement {}
impl DerivedFrom<Element> for HTMLBaseElement {}
impl DerivedFrom<HTMLElement> for HTMLBaseElement {}

impl HTMLBaseElement {
    pub fn new(local_name: LocalName, prefix: Option<Prefix>, document: Weak<Document>) -> Self {
        Self {
            htmlelement: HTMLElement::new_inherited(
                NodeTypeId::Element(ElementTypeId::HTMLElement(
                    HTMLElementTypeId::HTMLBaseElement,
                )),
                local_name,
                prefix,
                document,
            ),
        }
    }

    /// <https://html.spec.whatwg.org/multipage/#frozen-base-url>
    pub fn frozen_base_url(&self) -> BrowserUrl {
        let href = self
            .upcast::<Element>()
            .get_attribute(&ns!(), &local_name!("href"))
            .expect(
                "The frozen base url is only defined for base elements \
                 that have a base url.",
            );
        let document = document_from_node(self);
        let base = document.fallback_base_url();
        let parsed = base.join(&href.value.borrow());
        parsed.unwrap_or(base)
    }
}
impl VirtualMethods for HTMLBaseElement {
    fn super_type(&self) -> Option<&dyn VirtualMethods> {
        Some(self.upcast::<HTMLElement>() as &dyn VirtualMethods)
    }

    fn parse_plain_attribute(&self, name: &LocalName, value: String) -> AttrValue {
        match *name {
            local_name!("bgcolor") | local_name!("text") => {
                AttrValue::from_legacy_color(value.into())
            }
            local_name!("background") => {
                AttrValue::from_resolved_url(&document_from_node(self).base_url(), value.into())
            }
            _ => self
                .super_type()
                .unwrap()
                .parse_plain_attribute(name, value),
        }
    }
}
