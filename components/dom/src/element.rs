use std::rc::{Rc, Weak};

use html5ever::{ns, LocalName, Namespace, Prefix, QualName};

use crate::{attr::Attr, document::Document, node::Node, nodetype::NodeTypeId};
use html5ever::namespace_url;

#[derive(Clone)]
pub struct Element {
    node: Node,
    prefix: Option<Prefix>,
    local_name: LocalName,
    namespace: Namespace,
    attrs: Vec<Rc<Attr>>,
}

impl Element {
    pub fn new(
        node: Node,
        prefix: Option<Prefix>,
        local_name: LocalName,
        namespace: Namespace,
    ) -> Self {
        Self {
            node,
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

    pub fn set_attribute_from_parser(
        &self,
        qname: QualName,
        value: String,
        prefix: Option<Prefix>,
    ) {
        todo!()
    }

    pub fn create(
        node_type_id: NodeTypeId,
        name: QualName,
        is: Option<LocalName>,
        document: Weak<Document>,
    ) -> Element {
        let prefix = name.prefix.clone();
        match name.ns {
            ns!(html) => create_html_element(name, prefix, is, document),
            ns!(svg) => create_svg_element(name, prefix, document),
            _ => Element::new(
                Node::new(node_type_id, Some(document)),
                prefix,
                name.local,
                name.ns,
            ),
        }
    }
}

fn create_svg_element(
    name: QualName,
    prefix: Option<string_cache::Atom<html5ever::PrefixStaticSet>>,
    document: Weak<Document>,
) -> Element {
    todo!()
}

fn create_html_element(
    name: QualName,
    prefix: Option<string_cache::Atom<html5ever::PrefixStaticSet>>,
    is: Option<string_cache::Atom<html5ever::LocalNameStaticSet>>,
    document: Weak<Document>,
) -> Element {
    todo!()
}

impl crate::inheritance::Castable for Element {}
impl crate::inheritance::DerivedFrom<Node> for Element {}
