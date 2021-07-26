use std::{
    ops::Deref,
    rc::{Rc, Weak},
    slice::Iter,
};

use crate::{
    document::Document,
    error::{Error, ErrorResult, Fallible},
    inheritance::{Castable, DerivedFrom},
    nodetype::{CharacterDataTypeId, NodeTypeId},
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
    pub fn append_child(&self, node: Rc<Node>) -> Fallible<Rc<Node>> {
        Node::pre_insert(node, self, None)
    }

    pub fn insert_before(&self, node: Rc<Node>, child: Option<Rc<Node>>) -> Fallible<Rc<Node>> {
        Node::pre_insert(node, self, child)
    }

    pub fn node_type_id(&self) -> NodeTypeId {
        self.node_type_id
    }

    // https://dom.spec.whatwg.org/#dom-node-removechild
    pub fn remove_child(&self, node: &Node) -> Fallible<Rc<Node>> {
        Node::pre_remove(node, self)
    }

    pub fn get_next_sibling(&self) -> Option<Weak<Node>> {
        self.next_sibling.clone()
    }

    pub fn get_prev_sibling(&self) -> Option<Weak<Node>> {
        self.prev_sibling.clone()
    }

    // https://dom.spec.whatwg.org/#concept-tree-root
    pub fn get_root(&self) -> Rc<Node> {
        if self.parent_node.is_none() {
            Rc::new(self.clone())
        } else {
            self.parent_node
                .clone()
                .unwrap()
                .upgrade()
                .unwrap()
                .get_root()
        }
    }

    pub fn ancestors(&self) -> PrecedingNodeIterator {
        todo!()
    }

    pub fn is_ancestor_of(&self, node: &Node) -> bool {
        self.ancestors().any(|ancestor| ancestor.as_ptr() == node)
    }

    // https://dom.spec.whatwg.org/#concept-tree-host-including-inclusive-ancestor
    pub fn is_host_inclusive_ancestor_of(&self, parent: &Node) -> bool {
        // TODO: Step 1.2
        self.is_ancestor_of(parent)
    }

    // https://dom.spec.whatwg.org/#dom-node-firstchild
    pub fn get_first_child(&self) -> Option<Rc<Node>> {
        self.first_child.clone()
    }

    // https://dom.spec.whatwg.org/#concept-node-pre-insert
    pub fn pre_insert(
        node: Rc<Node>,
        parent: &Node,
        child: Option<Rc<Node>>,
    ) -> Fallible<Rc<Node>> {
        Node::ensure_pre_insertion_validity(node.clone(), parent, child.clone())?;

        let reference_child = match child {
            Some(child) if child == node => node.get_next_sibling().unwrap().upgrade().clone(),
            _ => child,
        };

        Node::insert(node.clone(), parent, reference_child);
        Ok(node)
    }

    // https://dom.spec.whatwg.org/#concept-node-insert
    fn insert(node: Rc<Node>, parent: &Node, child: Option<Rc<Node>>) {
        todo!()
    }

    // https://dom.spec.whatwg.org/#concept-node-ensure-pre-insertion-validity
    pub fn ensure_pre_insertion_validity(
        node: Rc<Node>,
        parent: &Node,
        child: Option<Rc<Node>>,
    ) -> ErrorResult {
        // Step 1
        match parent.node_type_id {
            NodeTypeId::Document | NodeTypeId::DocumentFragment(_) | NodeTypeId::Element(_) => (),
            _ => return Err(Error::HierarchyRequest),
        }

        // Step 2
        if node.is_host_inclusive_ancestor_of(parent) {
            return Err(Error::HierarchyRequest);
        }

        // Step 3
        if let Some(ref child) = child {
            if !parent.is_parent_of(child.clone()) {
                return Err(Error::NotFound);
            }
        }

        // Step 4
        match node.node_type_id {
            NodeTypeId::DocumentFragment(_)
            | NodeTypeId::DocumentType
            | NodeTypeId::Element(_)
            | NodeTypeId::CharacterData(_) => (),
            _ => return Err(Error::HierarchyRequest),
        }

        // Step 5
        match node.node_type_id {
            NodeTypeId::CharacterData(CharacterDataTypeId::Text(_)) => {
                if parent.node_type_id.is_document() {
                    return Err(Error::HierarchyRequest);
                }
            }
            NodeTypeId::DocumentType => {
                if !parent.node_type_id.is_document() {
                    return Err(Error::HierarchyRequest);
                }
            }
            _ => (),
        }

        // Step 6
        if parent.node_type_id == NodeTypeId::Document {
            match node.node_type_id {
                NodeTypeId::DocumentFragment(_) => {
                    if node
                        .children()
                        .any(|child_node| child_node.node_type_id.is_character_data_text())
                    {
                        return Err(Error::HierarchyRequest);
                    }

                    match node
                        .children()
                        .filter(|child_node| child_node.node_type_id.is_element())
                        .count()
                    {
                        0 => (),
                        1 => {
                            if parent
                                .children()
                                .any(|child_node| child_node.node_type_id.is_element())
                            {
                                return Err(Error::HierarchyRequest);
                            }
                            if child.is_some()
                                && child.as_ref().unwrap().node_type_id.is_document_type()
                            {
                                return Err(Error::HierarchyRequest);
                            }
                            if let Some(ref child) = child {
                                if child.following_siblings().any(|sibling| {
                                    sibling.upgrade().unwrap().node_type_id.is_document_type()
                                }) {
                                    return Err(Error::HierarchyRequest);
                                }
                            }
                        }
                        _ => return Err(Error::HierarchyRequest),
                    }
                }
                NodeTypeId::Element(_) => {}
                NodeTypeId::DocumentType => {
                    if parent
                        .children()
                        .any(|child_node| child_node.node_type_id.is_document_type())
                    {
                        return Err(Error::HierarchyRequest);
                    }
                    match child {
                        Some(ref child) => {
                            if child
                                .preceding_siblings()
                                .any(|sibling| sibling.upgrade().unwrap().node_type_id.is_element())
                            {
                                return Err(Error::HierarchyRequest);
                            }
                        }
                        None => {
                            if parent
                                .children()
                                .any(|child| child.node_type_id.is_element())
                            {
                                return Err(Error::HierarchyRequest);
                            }
                        }
                    }
                }
                _ => (),
            }
        }

        Ok(())
    }

    pub fn is_parent_of(&self, child: Rc<Node>) -> bool {
        child
            .parent_node
            .clone()
            .map_or(false, |parent| parent.as_ptr() == self)
    }

    // https://dom.spec.whatwg.org/#concept-node-pre-remove
    fn pre_remove(child: &Node, parent: &Node) -> Fallible<Rc<Node>> {
        todo!()
    }

    pub fn following_siblings(&self) -> FollowingNodeIterator {
        FollowingNodeIterator {
            current: self.next_sibling.clone(),
        }
    }

    pub fn preceding_siblings(&self) -> PrecedingNodeIterator {
        PrecedingNodeIterator {
            current: self.next_sibling.clone(),
        }
    }

    pub fn children(&self) -> Iter<Rc<Node>> {
        self.child_list.iter()
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

pub struct FollowingNodeIterator {
    current: Option<Weak<Node>>,
}
impl Iterator for FollowingNodeIterator {
    type Item = Weak<Node>;

    // https://dom.spec.whatwg.org/#concept-tree-following
    fn next(&mut self) -> Option<Self::Item> {
        match self.current.take() {
            Some(node) => {
                self.current = node.upgrade().unwrap().get_next_sibling();
                Some(node)
            }
            None => None,
        }
    }
}

pub struct PrecedingNodeIterator {
    current: Option<Weak<Node>>,
}
impl Iterator for PrecedingNodeIterator {
    type Item = Weak<Node>;

    // https://dom.spec.whatwg.org/#concept-tree-preceding
    fn next(&mut self) -> Option<Self::Item> {
        match self.current.take() {
            Some(node) => {
                self.current = node.upgrade().unwrap().get_prev_sibling();
                Some(node)
            }
            None => None,
        }
    }
}
