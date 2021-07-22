use std::rc::Weak;

use html5ever::{LocalName, Prefix};

use crate::{
    document::Document,
    element::Element,
    htmlelement::HTMLElement,
    node::Node,
    nodetype::{ElementTypeId, HTMLElementTypeId, NodeTypeId},
};

pub struct HTMLUnknownElement {
    htmlelement: HTMLElement,
}

impl crate::inheritance::Castable for HTMLUnknownElement {}
impl crate::inheritance::DerivedFrom<Node> for HTMLUnknownElement {}
impl crate::inheritance::DerivedFrom<Element> for HTMLUnknownElement {}
impl crate::inheritance::DerivedFrom<HTMLElement> for HTMLUnknownElement {}

impl HTMLUnknownElement {
    pub fn new(local_name: LocalName, prefix: Option<Prefix>, document: Weak<Document>) -> Self {
        Self {
            htmlelement: HTMLElement::new_inherited(
                NodeTypeId::Element(ElementTypeId::HTMLElement(
                    HTMLElementTypeId::HTMLUnknownElement,
                )),
                local_name,
                prefix,
                document,
            ),
        }
    }
}
