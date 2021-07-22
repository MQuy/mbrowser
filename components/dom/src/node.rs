use std::rc::{Rc, Weak};

use crate::{
    document::Document,
    error::{ErrorResult, Fallible},
    inheritance::{Castable, DerivedFrom},
    nodetype::NodeTypeId,
    virtualmethods::VirtualMethods,
};

#[derive(Clone)]
pub struct Node {
    node_type_id: NodeTypeId,
    parent_node: Option<Weak<Node>>,
    first_child: Option<Rc<Node>>,
    last_child: Option<Rc<Node>>,
    next_sibling: Option<Weak<Node>>,
    prev_sibling: Option<Weak<Node>>,
    child_list: Vec<Rc<Node>>,
    children_count: u32,
    owner_doc: Option<Weak<Document>>,
}
impl Castable for Node {}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

impl Node {
    pub fn new(node_type_id: NodeTypeId, doc: Option<Weak<Document>>) -> Node {
        Node {
            node_type_id,
            parent_node: Default::default(),
            first_child: Default::default(),
            last_child: Default::default(),
            next_sibling: Default::default(),
            prev_sibling: Default::default(),
            child_list: Default::default(),
            children_count: 0u32,
            owner_doc: doc,
        }
    }

    pub fn owner_doc(&self) -> Weak<Document> {
        self.owner_doc.clone().unwrap()
    }

    // https://dom.spec.whatwg.org/#dom-node-parentnode
    pub fn get_parent_node(&self) -> Option<Weak<Node>> {
        self.parent_node.clone()
    }

    // https://dom.spec.whatwg.org/#dom-node-appendchild
    pub fn append_child(&self, node: &Node) -> Fallible<Rc<Node>> {
        Node::pre_insert(node, self, None)
    }

    pub fn insert_before(&self, node: &Node, child: Option<&Node>) -> Fallible<Rc<Node>> {
        Node::pre_insert(node, self, child)
    }

    pub fn node_type_id(&self) -> NodeTypeId {
        self.node_type_id
    }

    // https://dom.spec.whatwg.org/#dom-node-removechild
    pub fn remove_child(&self, node: &Node) -> Fallible<Rc<Node>> {
        Node::pre_remove(node, self)
    }

    pub fn get_next_sibling(&self) -> Option<&Node> {
        todo!()
    }

    // https://dom.spec.whatwg.org/#dom-node-firstchild
    pub fn get_first_child(&self) -> Option<Rc<Node>> {
        self.first_child.clone()
    }

    // https://dom.spec.whatwg.org/#concept-node-pre-insert
    pub fn pre_insert(node: &Node, parent: &Node, child: Option<&Node>) -> Fallible<Rc<Node>> {
        Node::ensure_pre_insertion_validity(node, parent, child)?;

        let reference_child = match child {
            Some(child) if child == node => node.get_next_sibling(),
            _ => child,
        };

        Node::insert(node, parent, reference_child);
        Ok(Rc::new(node.clone()))
    }

    // https://dom.spec.whatwg.org/#concept-node-insert
    fn insert(node: &Node, parent: &Node, child: Option<&Node>) {
        todo!()
    }

    // https://dom.spec.whatwg.org/#concept-node-ensure-pre-insertion-validity
    pub fn ensure_pre_insertion_validity(
        node: &Node,
        parent: &Node,
        child: Option<&Node>,
    ) -> ErrorResult {
        todo!()
    }

    // https://dom.spec.whatwg.org/#concept-node-pre-remove
    fn pre_remove(child: &Node, parent: &Node) -> Fallible<Rc<Node>> {
        todo!()
    }
}

impl VirtualMethods for Node {
    fn super_type(&self) -> Option<&dyn VirtualMethods> {
        None
    }
}

pub fn document_from_node<T: DerivedFrom<Node>>(derived: &T) -> Weak<Document> {
    derived.upcast().owner_doc()
}
