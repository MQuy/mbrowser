use std::rc::Weak;

use crate::{
    document::Document,
    inheritance::{Castable, DerivedFrom},
    node::Node,
    nodetype::{DocumentFragmentTypeId, NodeTypeId},
};

#[derive(Clone)]
pub struct DocumentFragment {
    node: Node,
}

impl DocumentFragment {
    pub fn new(document: Weak<Document>) -> Self {
        Self {
            node: Node::new(
                NodeTypeId::DocumentFragment(DocumentFragmentTypeId::DocumentFragment),
                Some(document),
            ),
        }
    }
}

impl Castable for DocumentFragment {}
impl DerivedFrom<Node> for DocumentFragment {}
