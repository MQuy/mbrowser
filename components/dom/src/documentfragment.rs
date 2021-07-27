use std::rc::Rc;

use crate::{
    document::Document,
    inheritance::{Castable, DerivedFrom},
    node::Node,
    nodetype::{DocumentFragmentTypeId, NodeTypeId},
};

#[derive(Clone)]
#[repr(C)]
pub struct DocumentFragment {
    node: Node,
}

impl DocumentFragment {
    pub fn new(document: Rc<Document>) -> Self {
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
