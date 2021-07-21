use std::rc::Weak;

use crate::{
    element::Element,
    node::Node,
    nodetype::{ElementTypeId, NodeTypeId, SVGElementTypeId, SVGGraphicsElementTypeId},
    svgelement::SVGElement,
};
use html5ever::{LocalName, Namespace, Prefix};

use crate::document::Document;

pub struct SVGGraphicsElement {
    svgelement: SVGElement,
}

impl SVGGraphicsElement {
    pub fn new_inherited(
        node_type_id: NodeTypeId,
        prefix: Option<Prefix>,
        local_name: LocalName,
        namespace: Namespace,
        document: Weak<Document>,
    ) -> Self {
        Self {
            svgelement: SVGElement::new_inherited(
                node_type_id,
                prefix,
                local_name,
                namespace,
                document,
            ),
        }
    }
}

impl crate::inheritance::Castable for SVGGraphicsElement {}
impl crate::inheritance::DerivedFrom<Node> for SVGGraphicsElement {}
impl crate::inheritance::DerivedFrom<Element> for SVGGraphicsElement {}
impl crate::inheritance::DerivedFrom<SVGElement> for SVGGraphicsElement {}
