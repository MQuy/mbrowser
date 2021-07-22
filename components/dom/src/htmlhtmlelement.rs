use std::rc::Weak;

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
pub struct HTMLHtmlElement {
    htmlelement: HTMLElement,
}
impl Castable for HTMLHtmlElement {}
impl DerivedFrom<Node> for HTMLHtmlElement {}
impl DerivedFrom<Element> for HTMLHtmlElement {}
impl DerivedFrom<HTMLElement> for HTMLHtmlElement {}

impl HTMLHtmlElement {
    pub fn new(local_name: LocalName, prefix: Option<Prefix>, document: Weak<Document>) -> Self {
        Self {
            htmlelement: HTMLElement::new_inherited(
                NodeTypeId::Element(ElementTypeId::HTMLElement(
                    HTMLElementTypeId::HTMLHtmlElement,
                )),
                local_name,
                prefix,
                document,
            ),
        }
    }
}