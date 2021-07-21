use std::rc::Weak;

use html5ever::{LocalName, Namespace, Prefix};

use crate::{
    document::Document,
    element::Element,
    node::Node,
    nodetype::{ElementTypeId, NodeTypeId, SVGElementTypeId, SVGGraphicsElementTypeId},
    svgelement::SVGElement,
    svggraphicselement::SVGGraphicsElement,
};

pub struct SVGSVGElement {
    svggraphicselement: SVGGraphicsElement,
}

impl SVGSVGElement {
    pub fn new(
        prefix: Option<Prefix>,
        local_name: LocalName,
        namespace: Namespace,
        document: Weak<Document>,
    ) -> Self {
        Self {
            svggraphicselement: SVGGraphicsElement::new_inherited(
                NodeTypeId::Element(ElementTypeId::SVGElement(
                    SVGElementTypeId::SVGGraphicsElement(SVGGraphicsElementTypeId::SVGSVGElement),
                )),
                prefix,
                local_name,
                namespace,
                document,
            ),
        }
    }
}

impl crate::inheritance::Castable for SVGSVGElement {}
impl crate::inheritance::DerivedFrom<Node> for SVGSVGElement {}
impl crate::inheritance::DerivedFrom<Element> for SVGSVGElement {}
impl crate::inheritance::DerivedFrom<SVGElement> for SVGSVGElement {}
impl crate::inheritance::DerivedFrom<SVGGraphicsElement> for SVGSVGElement {}
