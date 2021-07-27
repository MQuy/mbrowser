use std::rc::Rc;

use html5ever::{LocalName, Prefix};

use crate::{
    document::Document,
    element::Element,
    htmlelement::HTMLElement,
    inheritance::{Castable, DerivedFrom},
    node::Node,
    nodetype::{ElementTypeId, HTMLElementTypeId, NodeTypeId},
};

#[derive(Clone)]
#[repr(C)]
pub struct HTMLHeadElement {
    htmlelement: HTMLElement,
}

impl Castable for HTMLHeadElement {}
impl DerivedFrom<Node> for HTMLHeadElement {}
impl DerivedFrom<Element> for HTMLHeadElement {}
impl DerivedFrom<HTMLElement> for HTMLHeadElement {}

impl HTMLHeadElement {
    pub fn new(local_name: LocalName, prefix: Option<Prefix>, document: Rc<Document>) -> Self {
        Self {
            htmlelement: HTMLElement::new_inherited(
                NodeTypeId::Element(ElementTypeId::HTMLElement(
                    HTMLElementTypeId::HTMLHeadElement,
                )),
                local_name,
                prefix,
                document,
            ),
        }
    }
}
