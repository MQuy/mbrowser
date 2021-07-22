use std::rc::Weak;

use crate::{
    element::Element, inheritance::Castable, node::Node, nodetype::NodeTypeId,
    svgelement::SVGElement, virtualmethods::VirtualMethods,
};
use html5ever::{LocalName, Prefix};

use crate::document::Document;

pub struct SVGGraphicsElement {
    svgelement: SVGElement,
}

impl crate::inheritance::Castable for SVGGraphicsElement {}
impl crate::inheritance::DerivedFrom<Node> for SVGGraphicsElement {}
impl crate::inheritance::DerivedFrom<Element> for SVGGraphicsElement {}
impl crate::inheritance::DerivedFrom<SVGElement> for SVGGraphicsElement {}

impl SVGGraphicsElement {
    pub fn new_inherited(
        node_type_id: NodeTypeId,
        local_name: LocalName,
        prefix: Option<Prefix>,
        document: Weak<Document>,
    ) -> Self {
        Self {
            svgelement: SVGElement::new_inherited(node_type_id, local_name, prefix, document),
        }
    }
}

impl VirtualMethods for SVGGraphicsElement {
    fn super_type(&self) -> Option<&dyn VirtualMethods> {
        Some(self.upcast::<SVGElement>() as &dyn VirtualMethods)
    }
}
