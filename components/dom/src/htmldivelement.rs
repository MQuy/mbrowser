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
pub struct HTMLDivElement {
    htmlelement: HTMLElement,
}

impl HTMLDivElement {
    pub fn new(local_name: LocalName, prefix: Option<Prefix>, document: Rc<Document>) -> Self {
        Self {
            htmlelement: HTMLElement::new_inherited(
                NodeTypeId::Element(ElementTypeId::HTMLElement(
                    HTMLElementTypeId::HTMLDivElement,
                )),
                local_name,
                prefix,
                document,
            ),
        }
    }
}

impl Castable for HTMLDivElement {}
impl DerivedFrom<Node> for HTMLDivElement {}
impl DerivedFrom<Element> for HTMLDivElement {}
impl DerivedFrom<HTMLElement> for HTMLDivElement {}
