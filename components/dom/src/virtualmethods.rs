use std::rc::Rc;

use crate::{document::Document, inheritance::Castable};
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

    /// Called when a Node is removed from a tree, where 'tree_connected'
    /// indicates whether the tree is part of a Document.
    /// Implements removing steps:
    /// <https://dom.spec.whatwg.org/#concept-node-remove-ext>
    fn unbind_from_tree(&self) {
        if let Some(ref s) = self.super_type() {
            s.unbind_from_tree();
        }
    }

    /// <https://dom.spec.whatwg.org/#concept-node-adopt-ext>
    fn adopting_steps(&self, old_doc: Rc<Document>) {
        if let Some(ref s) = self.super_type() {
            s.adopting_steps(old_doc);
        }
    }

    /// Called on an element when it is popped off the stack of open elements
    /// of a parser.
    fn pop(&self) {
        if let Some(ref s) = self.super_type() {
            s.pop();
        }
    }
}

/// Obtain a VirtualMethods instance for a given Node-derived object. Any
/// method call on the trait object will invoke the corresponding method on the
/// concrete type, propagating up the parent hierarchy unless otherwise
/// interrupted.
pub fn vtable_for(node: &Node) -> &dyn VirtualMethods {
    match node.get_node_type_id() {
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
