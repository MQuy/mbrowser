use std::rc::{Rc, Weak};

use crate::{document::Document, inheritance::Castable, nodetype::NodeTypeId};

#[derive(Clone)]
pub struct Node {
    type_id: NodeTypeId,
    parent_node: Weak<Node>,
    first_child: Option<Rc<Node>>,
    last_child: Option<Rc<Node>>,
    next_sibling: Weak<Node>,
    prev_sibling: Weak<Node>,
    child_list: Vec<Rc<Node>>,
    children_count: u32,
}

impl Node {
    pub fn new(type_id: NodeTypeId, doc: Option<&Document>) -> Node {
        Node {
            type_id,
            parent_node: Default::default(),
            first_child: Default::default(),
            last_child: Default::default(),
            next_sibling: Default::default(),
            prev_sibling: Default::default(),
            child_list: Default::default(),
            children_count: 0u32,
        }
    }
}
impl Castable for Node {}
