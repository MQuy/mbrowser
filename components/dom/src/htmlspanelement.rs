use std::rc::Rc;

use html5ever::{LocalName, Prefix};

use crate::document::Document;
use crate::element::Element;
use crate::htmlelement::HTMLElement;
use crate::node::Node;
use crate::nodetype::{ElementTypeId, HTMLElementTypeId, NodeTypeId};

#[derive(Clone)]
#[repr(C)]
pub struct HTMLSpanElement {
	htmlelement: HTMLElement,
}

impl crate::inheritance::Castable for HTMLSpanElement {}
impl crate::inheritance::DerivedFrom<Node> for HTMLSpanElement {}
impl crate::inheritance::DerivedFrom<Element> for HTMLSpanElement {}
impl crate::inheritance::DerivedFrom<HTMLElement> for HTMLSpanElement {}

impl HTMLSpanElement {
	pub fn new(local_name: LocalName, prefix: Option<Prefix>, document: Rc<Document>) -> Self {
		Self {
			htmlelement: HTMLElement::new_inherited(
				NodeTypeId::Element(ElementTypeId::HTMLElement(
					HTMLElementTypeId::HTMLSpanElement,
				)),
				local_name,
				prefix,
				document,
			),
		}
	}
}
