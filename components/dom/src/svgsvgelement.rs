use std::rc::Rc;

use html5ever::{local_name, LocalName, Prefix};

use crate::{
    attr::AttrValue,
    document::Document,
    element::Element,
    inheritance::Castable,
    node::Node,
    nodetype::{ElementTypeId, NodeTypeId, SVGElementTypeId, SVGGraphicsElementTypeId},
    svgelement::SVGElement,
    svggraphicselement::SVGGraphicsElement,
    virtualmethods::VirtualMethods,
};

const DEFAULT_WIDTH: u32 = 300;
const DEFAULT_HEIGHT: u32 = 150;

#[derive(Clone)]
#[repr(C)]
pub struct SVGSVGElement {
    svggraphicselement: SVGGraphicsElement,
}

impl crate::inheritance::Castable for SVGSVGElement {}
impl crate::inheritance::DerivedFrom<Node> for SVGSVGElement {}
impl crate::inheritance::DerivedFrom<Element> for SVGSVGElement {}
impl crate::inheritance::DerivedFrom<SVGElement> for SVGSVGElement {}
impl crate::inheritance::DerivedFrom<SVGGraphicsElement> for SVGSVGElement {}

impl SVGSVGElement {
    pub fn new(local_name: LocalName, prefix: Option<Prefix>, document: Rc<Document>) -> Self {
        Self {
            svggraphicselement: SVGGraphicsElement::new_inherited(
                NodeTypeId::Element(ElementTypeId::SVGElement(
                    SVGElementTypeId::SVGGraphicsElement(SVGGraphicsElementTypeId::SVGSVGElement),
                )),
                local_name,
                prefix,
                document,
            ),
        }
    }
}

impl VirtualMethods for SVGSVGElement {
    fn super_type(&self) -> Option<&dyn VirtualMethods> {
        Some(self.upcast::<SVGGraphicsElement>() as &dyn VirtualMethods)
    }

    fn parse_plain_attribute(&self, name: &LocalName, value: String) -> AttrValue {
        match name {
            &local_name!("width") => AttrValue::from_u32(value.into(), DEFAULT_WIDTH),
            &local_name!("height") => AttrValue::from_u32(value.into(), DEFAULT_HEIGHT),
            _ => self
                .super_type()
                .unwrap()
                .parse_plain_attribute(name, value),
        }
    }
}
