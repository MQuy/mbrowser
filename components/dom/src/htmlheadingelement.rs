use std::rc::Rc;

use html5ever::{LocalName, Prefix};

use crate::document::Document;
use crate::element::Element;
use crate::htmlelement::HTMLElement;
use crate::inheritance::{Castable, DerivedFrom};
use crate::node::Node;
use crate::nodetype::{ElementTypeId, HTMLElementTypeId, NodeTypeId};

#[derive(Clone)]
pub enum HeadingLevel {
	Heading1,
	Heading2,
	Heading3,
	Heading4,
	Heading5,
	Heading6,
}

#[derive(Clone)]
#[repr(C)]
pub struct HTMLHeadingElement {
	htmlelement: HTMLElement,
	level: HeadingLevel,
}

impl HTMLHeadingElement {
	pub fn new(local_name: LocalName, prefix: Option<Prefix>, document: Rc<Document>, level: HeadingLevel) -> Self {
		Self {
			htmlelement: HTMLElement::new_inherited(
				NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLHeadingElement)),
				local_name,
				prefix,
				document,
			),
			level,
		}
	}
}

impl Castable for HTMLHeadingElement {}
impl DerivedFrom<Node> for HTMLHeadingElement {}
impl DerivedFrom<Element> for HTMLHeadingElement {}
impl DerivedFrom<HTMLElement> for HTMLHeadingElement {}
