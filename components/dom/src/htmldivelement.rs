use std::rc::Rc;

use html5ever::{LocalName, Prefix};

use crate::document::Document;
use crate::element::Element;
use crate::htmlelement::HTMLElement;
use crate::inheritance::{Castable, DerivedFrom};
use crate::node::Node;
use crate::nodetype::{ElementTypeId, HTMLElementTypeId, NodeTypeId};

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
