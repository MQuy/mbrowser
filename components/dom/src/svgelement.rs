use std::rc::Weak;

use html5ever::{LocalName, Namespace, Prefix};

use crate::{
    document::Document,
    element::Element,
    inheritance::{Castable, DerivedFrom},
    node::Node,
    nodetype::{ElementTypeId, NodeTypeId, SVGElementTypeId},
};

#[derive(Clone)]
pub struct SVGElement {
    element: Element,
}

impl Castable for SVGElement {}
impl DerivedFrom<Node> for SVGElement {}
impl DerivedFrom<Element> for SVGElement {}

impl SVGElement {
    pub fn new(
        prefix: Option<Prefix>,
        local_name: LocalName,
        namespace: Namespace,
        document: Weak<Document>,
    ) -> Self {
        SVGElement::new_inherited(
            NodeTypeId::Element(ElementTypeId::SVGElement(SVGElementTypeId::SVGElement)),
            prefix,
            local_name,
            namespace,
            document,
        )
    }

    pub fn new_inherited(
        node_type_id: NodeTypeId,
        prefix: Option<Prefix>,
        local_name: LocalName,
        namespace: Namespace,
        document: Weak<Document>,
    ) -> Self {
        Self {
            element: Element::new_inherited(node_type_id, prefix, local_name, namespace, document),
        }
    }
}
