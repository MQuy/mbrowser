use crate::inheritance::Castable;
use html5ever::LocalName;

use crate::{
    attr::AttrValue,
    element::Element,
    htmlbodyelement::HTMLBodyElement,
    htmlelement::HTMLElement,
    node::Node,
    nodetype::{
        ElementTypeId, HTMLElementTypeId, NodeTypeId, SVGElementTypeId, SVGGraphicsElementTypeId,
    },
    svgelement::SVGElement,
    svgsvgelement::SVGSVGElement,
};

/// Trait to allow DOM nodes to opt-in to overriding (or adding to) common
/// behaviours. Replicates the effect of C++ virtual methods.
pub trait VirtualMethods {
    /// Returns self as the superclass of the implementation for this trait,
    /// if any.
    fn super_type(&self) -> Option<&dyn VirtualMethods>;

    /// Returns the right AttrValue variant for the attribute with name `name`
    /// on this element.
    fn parse_plain_attribute(&self, name: &LocalName, value: String) -> AttrValue {
        match self.super_type() {
            Some(ref s) => s.parse_plain_attribute(name, value),
            _ => AttrValue::String(value.into()),
        }
    }
}

/// Obtain a VirtualMethods instance for a given Node-derived object. Any
/// method call on the trait object will invoke the corresponding method on the
/// concrete type, propagating up the parent hierarchy unless otherwise
/// interrupted.
pub fn vtable_for(node: &Node) -> &dyn VirtualMethods {
    match node.node_type_id() {
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLBodyElement)) => {
            node.downcast::<HTMLBodyElement>() as &dyn VirtualMethods
        }
        NodeTypeId::Element(ElementTypeId::SVGElement(SVGElementTypeId::SVGGraphicsElement(
            SVGGraphicsElementTypeId::SVGSVGElement,
        ))) => node.downcast::<SVGSVGElement>() as &dyn VirtualMethods,
        NodeTypeId::Element(ElementTypeId::SVGElement(SVGElementTypeId::SVGElement)) => {
            node.downcast::<SVGElement>() as &dyn VirtualMethods
        }
        NodeTypeId::Element(ElementTypeId::Element) => {
            node.downcast::<Element>() as &dyn VirtualMethods
        }
        NodeTypeId::Element(_) => node.downcast::<HTMLElement>() as &dyn VirtualMethods,
        _ => node as &dyn VirtualMethods,
    }
}
