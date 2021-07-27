use std::rc::Rc;

use html5ever::{LocalName, Prefix};

use crate::document::Document;
use crate::element::Element;
use crate::htmlelement::HTMLElement;
use crate::node::Node;
use crate::nodetype::{ElementTypeId, HTMLElementTypeId, NodeTypeId};

#[derive(Clone)]
#[repr(C)]
pub struct HTMLUnknownElement {
    htmlelement: HTMLElement,
}

impl crate::inheritance::Castable for HTMLUnknownElement {}
impl crate::inheritance::DerivedFrom<Node> for HTMLUnknownElement {}
impl crate::inheritance::DerivedFrom<Element> for HTMLUnknownElement {}
impl crate::inheritance::DerivedFrom<HTMLElement> for HTMLUnknownElement {}

impl HTMLUnknownElement {
    pub fn new(local_name: LocalName, prefix: Option<Prefix>, document: Rc<Document>) -> Self {
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
