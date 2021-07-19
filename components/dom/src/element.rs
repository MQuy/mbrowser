use html5ever::{LocalName, Namespace, Prefix};

use crate::node::Node;

#[derive(Clone)]
pub struct Element {
    node: Node,
    prefix: Option<Prefix>,
    local_name: LocalName,
    tag_name: LocalName,
    namespace: Namespace,
}

impl Element {
    pub fn namespace(&self) -> &Namespace {
        &self.namespace
    }

    pub fn local_name(&self) -> &LocalName {
        &self.local_name
    }
}

impl crate::inheritance::Castable for Element {}
impl crate::inheritance::DerivedFrom<Node> for Element {}
