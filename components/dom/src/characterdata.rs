use std::rc::Rc;

use crate::{
    document::Document,
    inheritance::{Castable, DerivedFrom},
    node::Node,
    nodetype::NodeTypeId,
};

#[derive(Clone)]
#[repr(C)]
pub struct CharacterData {
    node: Node,
    data: String,
}

impl CharacterData {
    pub fn new_inherited(node_type_id: NodeTypeId, data: String, document: Rc<Document>) -> Self {
        Self {
            node: Node::new(node_type_id, Some(document)),
            data,
        }
    }
}

impl Castable for CharacterData {}
impl DerivedFrom<Node> for CharacterData {}
