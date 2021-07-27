use std::rc::Rc;

use html5ever::{local_name, LocalName, Prefix};

use crate::{
    attr::AttrValue,
    document::Document,
    element::Element,
    htmlelement::HTMLElement,
    inheritance::{Castable, DerivedFrom},
    node::{document_from_node, Node},
    nodetype::{ElementTypeId, HTMLElementTypeId, NodeTypeId},
    virtualmethods::VirtualMethods,
};

#[derive(Clone)]
#[repr(C)]
pub struct HTMLBodyElement {
    htmlelement: HTMLElement,
}

impl Castable for HTMLBodyElement {}
impl DerivedFrom<Node> for HTMLBodyElement {}
impl DerivedFrom<Element> for HTMLBodyElement {}
impl DerivedFrom<HTMLElement> for HTMLBodyElement {}

impl HTMLBodyElement {
    pub fn new(local_name: LocalName, prefix: Option<Prefix>, document: Rc<Document>) -> Self {
        Self {
            htmlelement: HTMLElement::new_inherited(
                NodeTypeId::Element(ElementTypeId::HTMLElement(
                    HTMLElementTypeId::HTMLBodyElement,
                )),
                local_name,
                prefix,
                document,
            ),
        }
    }
}
impl VirtualMethods for HTMLBodyElement {
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
