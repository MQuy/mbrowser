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
pub struct HTMLHtmlElement {
	htmlelement: HTMLElement,
}
impl Castable for HTMLHtmlElement {}
impl DerivedFrom<Node> for HTMLHtmlElement {}
impl DerivedFrom<Element> for HTMLHtmlElement {}
impl DerivedFrom<HTMLElement> for HTMLHtmlElement {}

impl HTMLHtmlElement {
	pub fn new(local_name: LocalName, prefix: Option<Prefix>, document: Rc<Document>) -> Self {
		Self {
			htmlelement: HTMLElement::new_inherited(
				NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLHtmlElement)),
				local_name,
				prefix,
				document,
			),
		}
	}
}
