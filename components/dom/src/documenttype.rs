use std::rc::Rc;

use crate::document::Document;
use crate::node::Node;

#[derive(Clone)]
#[repr(C)]
pub struct DocumentType {
    node: Node,
    name: String,
    public_id: String,
    system_id: String,
}

impl DocumentType {
    pub fn new(name: String, public_id: String, system_id: String, document: Rc<Document>) -> Self {
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
