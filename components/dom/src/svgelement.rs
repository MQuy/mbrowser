use std::rc::Rc;

use html5ever::{namespace_url, ns, LocalName, Prefix};

use crate::document::Document;
use crate::element::Element;
use crate::inheritance::{Castable, DerivedFrom};
use crate::node::Node;
use crate::nodetype::{ElementTypeId, NodeTypeId, SVGElementTypeId};
use crate::virtualmethods::VirtualMethods;

#[derive(Clone)]
#[repr(C)]
pub struct SVGElement {
	element: Element,
}

impl Castable for SVGElement {}
impl DerivedFrom<Node> for SVGElement {}
impl DerivedFrom<Element> for SVGElement {}

impl SVGElement {
	pub fn new(local_name: LocalName, prefix: Option<Prefix>, document: Rc<Document>) -> Self {
		SVGElement::new_inherited(
			NodeTypeId::Element(ElementTypeId::SVGElement(SVGElementTypeId::SVGElement)),
			local_name,
			prefix,
			document,
		)
	}

	pub fn new_inherited(
		node_type_id: NodeTypeId,
		local_name: LocalName,
		prefix: Option<Prefix>,
		document: Rc<Document>,
	) -> Self {
		Self {
			element: Element::new_inherited(node_type_id, local_name, ns!(svg), prefix, document),
		}
	}
}

impl VirtualMethods for SVGElement {
	fn super_type(&self) -> Option<&dyn VirtualMethods> {
		Some(self.upcast::<Element>() as &dyn VirtualMethods)
	}
}
