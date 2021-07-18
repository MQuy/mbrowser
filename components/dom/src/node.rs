use std::rc::{Rc, Weak};

#[derive(Clone)]
pub struct Node {
    parent_node: Weak<Node>,
    first_child: Rc<Node>,
    last_child: Rc<Node>,
    next_sibling: Weak<Node>,
    prev_sibling: Weak<Node>,
    child_list: Vec<Rc<Node>>,
    children_count: u32,
}
