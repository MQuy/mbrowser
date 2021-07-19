use dom_struct::dom_struct;
use std::rc::{Rc, Weak};

use crate::nodetype::NodeTypeId;

#[dom_struct]
pub struct Node {
    type_id: NodeTypeId,
    parent_node: Weak<Node>,
    first_child: Rc<Node>,
    last_child: Rc<Node>,
    next_sibling: Weak<Node>,
    prev_sibling: Weak<Node>,
    child_list: Vec<Rc<Node>>,
    children_count: u32,
}
