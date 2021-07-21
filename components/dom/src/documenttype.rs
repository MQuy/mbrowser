use std::rc::Weak;

use crate::{document::Document, node::Node};

pub struct DocumentType {
    node: Node,
    name: String,
    public_id: String,
    system_id: String,
}

impl DocumentType {
    pub fn new(
        name: String,
        public_id: String,
        system_id: String,
        document: Weak<Document>,
    ) -> Self {
        DocumentType {
            node: Node::new(crate::nodetype::NodeTypeId::DocumentType, Some(document)),
            name,
            public_id,
            system_id,
        }
    }
}

impl crate::inheritance::Castable for DocumentType {}
impl crate::inheritance::DerivedFrom<Node> for DocumentType {}
