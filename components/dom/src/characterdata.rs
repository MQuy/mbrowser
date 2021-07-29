use std::cell::{Ref, RefCell};
use std::rc::Rc;

use crate::document::Document;
use crate::inheritance::{Castable, DerivedFrom};
use crate::node::Node;
use crate::nodetype::NodeTypeId;

#[derive(Clone)]
#[repr(C)]
pub struct CharacterData {
    node: Node,
    data: RefCell<String>,
}

impl CharacterData {
    pub fn new_inherited(node_type_id: NodeTypeId, data: String, document: Rc<Document>) -> Self {
        Self {
            node: Node::new(node_type_id, Some(document)),
            data: RefCell::new(data),
        }
    }

    #[inline]
    pub fn data(&self) -> Ref<String> {
        self.data.borrow()
    }

    #[inline]
    pub fn append_data(&self, data: &str) {
        self.data.borrow_mut().push_str(data);
    }
}

impl Castable for CharacterData {}
impl DerivedFrom<Node> for CharacterData {}
