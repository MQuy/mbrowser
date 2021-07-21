use std::rc::{Rc, Weak};

use html5ever::{local_name, ns, LocalName, Namespace, Prefix, QualName};

use crate::{
    attr::Attr,
    document::Document,
    inheritance::Castable,
    node::Node,
    nodetype::{ElementTypeId, NodeTypeId},
    svgelement::SVGElement,
    svgsvgelement::SVGSVGElement,
};
use html5ever::namespace_url;

#[derive(Clone)]
pub struct Element {
    node: Node,
    prefix: Option<Prefix>,
    local_name: LocalName,
    namespace: Namespace,
    attrs: Vec<Rc<Attr>>,
}

impl crate::inheritance::Castable for Element {}
impl crate::inheritance::DerivedFrom<Node> for Element {}

impl Element {
    pub fn new(
        prefix: Option<Prefix>,
        local_name: LocalName,
        namespace: Namespace,
        document: Weak<Document>,
    ) -> Self {
        Element::new_inherited(
            NodeTypeId::Element(ElementTypeId::Element),
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
            node: Node::new(node_type_id, Some(document)),
            prefix,
            local_name,
            namespace,
            attrs: Vec::new(),
        }
    }

    pub fn namespace(&self) -> &Namespace {
        &self.namespace
    }

    pub fn local_name(&self) -> &LocalName {
        &self.local_name
    }

    pub fn set_attribute(&self, qname: QualName, value: String, prefix: Option<Prefix>) {
        todo!()
    }

    pub fn create(name: QualName, is: Option<LocalName>, document: Weak<Document>) -> Rc<Element> {
        let prefix = name.prefix.clone();
        match name.ns {
            ns!(html) => create_html_element(name, prefix, is, document),
            ns!(svg) => create_svg_element(name, prefix, document),
            _ => todo!(),
        }
    }
}

fn create_svg_element(
    name: QualName,
    prefix: Option<string_cache::Atom<html5ever::PrefixStaticSet>>,
    document: Weak<Document>,
) -> Rc<Element> {
    match name.local {
        local_name!("svg") => {
            let element = SVGSVGElement::new(prefix, name.local, name.ns, document);
            Rc::new(element.upcast::<Element>().clone())
        }
        _ => {
            let element = SVGElement::new(prefix, name.local, name.ns, document);
            Rc::new(element.upcast::<Element>().clone())
        }
    }
}

// https://dom.spec.whatwg.org/#concept-create-element
fn create_html_element(
    name: QualName,
    prefix: Option<string_cache::Atom<html5ever::PrefixStaticSet>>,
    is: Option<string_cache::Atom<html5ever::LocalNameStaticSet>>,
    document: Weak<Document>,
) -> Rc<Element> {
    todo!()
}
