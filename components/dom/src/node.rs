use std::cell::RefCell;
use std::ops::Deref;
use std::rc::{Rc, Weak};

use crate::document::Document;
use crate::error::{Error, ErrorResult, Fallible};
use crate::inheritance::{Castable, DerivedFrom};
use crate::nodetype::{CharacterDataTypeId, NodeTypeId};
use crate::virtualmethods::{vtable_for, VirtualMethods};

#[derive(Clone)]
pub struct Node {
    node_type_id: NodeTypeId,
    parent_node: RefCell<Option<Weak<Node>>>,
    first_child: RefCell<Option<Rc<Node>>>,
    last_child: RefCell<Option<Rc<Node>>>,
    next_sibling: RefCell<Option<Weak<Node>>>,
    prev_sibling: RefCell<Option<Weak<Node>>>,
    children_count: u32,
    owner_doc: RefCell<Option<Weak<Document>>>,
}
impl Castable for Node {}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

impl Node {
    pub fn new(node_type_id: NodeTypeId, doc: Option<Rc<Document>>) -> Node {
        Node {
            node_type_id,
            parent_node: Default::default(),
            first_child: Default::default(),
            last_child: Default::default(),
            next_sibling: Default::default(),
            prev_sibling: Default::default(),
            children_count: 0u32,
            owner_doc: RefCell::new(match doc {
                Some(doc) => Some(Rc::downgrade(&doc)),
                None => None,
            }),
        }
    }

    pub fn get_owner_doc(&self) -> Option<Rc<Document>> {
        match self.owner_doc.borrow().deref() {
            Some(node) => node.upgrade(),
            _ => None,
        }
    }

    pub fn set_owner_doc(&self, document: Rc<Document>) {
        self.owner_doc.replace(Some(Rc::downgrade(&document)));
    }

    // https://dom.spec.whatwg.org/#dom-node-parentnode
    pub fn get_parent_node(&self) -> Option<Rc<Node>> {
        match self.parent_node.borrow().deref() {
            Some(node) => node.upgrade(),
            _ => None,
        }
    }

    pub fn get_next_sibling(&self) -> Option<Rc<Node>> {
        match self.next_sibling.borrow().deref() {
            Some(node) => node.upgrade(),
            _ => None,
        }
    }

    pub fn get_prev_sibling(&self) -> Option<Rc<Node>> {
        match self.prev_sibling.borrow().deref() {
            Some(node) => node.upgrade(),
            _ => None,
        }
    }

    pub fn get_node_type_id(&self) -> NodeTypeId {
        self.node_type_id
    }

    // https://dom.spec.whatwg.org/#dom-node-firstchild
    pub fn get_first_child(&self) -> Option<Rc<Node>> {
        self.first_child.borrow().deref().clone()
    }

    // https://dom.spec.whatwg.org/#dom-node-lastchild
    pub fn get_last_child(&self) -> Option<Rc<Node>> {
        self.last_child.borrow().deref().clone()
    }

    fn add_child(&self, new_child: Rc<Node>, before: Option<Rc<Node>>) {
        new_child
            .parent_node
            .replace(Some(Rc::downgrade(&Rc::new(self.clone()))));

        match before {
            Some(ref before) => {
                match before.get_prev_sibling() {
                    Some(ref prev_sibling) => {
                        prev_sibling
                            .next_sibling
                            .replace(Some(Rc::downgrade(&new_child)));
                        new_child
                            .prev_sibling
                            .replace(Some(Rc::downgrade(prev_sibling)));
                    },
                    None => {
                        self.first_child.replace(Some(new_child.clone()));
                    },
                }
                before.prev_sibling.replace(Some(Rc::downgrade(&new_child)));
                new_child.next_sibling.replace(Some(Rc::downgrade(before)));
            },
            None => {
                match self.get_last_child() {
                    Some(ref last_child) => {
                        last_child
                            .next_sibling
                            .replace(Some(Rc::downgrade(&new_child)));
                        new_child
                            .prev_sibling
                            .replace(Some(Rc::downgrade(last_child)));
                    },
                    None => {
                        self.first_child.replace(Some(new_child.clone()));
                    },
                };
                self.last_child.replace(Some(new_child.clone()));
            },
        }
    }
    // https://dom.spec.whatwg.org/#dom-node-appendchild
    pub fn append_child(&self, node: Rc<Node>) -> Fallible<Rc<Node>> {
        Node::pre_insert(node, self, None)
    }

    // https://dom.spec.whatwg.org/#dom-node-insertbefore
    pub fn insert_before(&self, node: Rc<Node>, child: Option<Rc<Node>>) -> Fallible<Rc<Node>> {
        Node::pre_insert(node, self, child)
    }

    // https://dom.spec.whatwg.org/#dom-node-removechild
    pub fn remove_child(&self, node: Rc<Node>) -> Fallible<Rc<Node>> {
        Node::pre_remove(node, self)
    }

    // https://dom.spec.whatwg.org/#concept-tree-root
    pub fn get_root(&self) -> Rc<Node> {
        let parent_node = self.get_parent_node();
        if parent_node.is_none() {
            Rc::new(self.clone())
        } else {
            parent_node.clone().unwrap().get_root()
        }
    }

    pub fn ancestors(&self) -> impl Iterator<Item = Rc<Node>> {
        SimpleNodeIterator {
            current: self.get_parent_node(),
            next_node: |n: &Rc<Node>| n.get_parent_node(),
        }
    }

    pub fn is_ancestor_of(&self, node: &Node) -> bool {
        self.ancestors().any(|ancestor| ancestor.as_ref() == node)
    }

    /// Iterates over this node and all its descendants, in preorder.
    pub fn traverse_preorder(&self) -> TreeIterator {
        TreeIterator::new(Rc::new(self.clone()))
    }

    // https://dom.spec.whatwg.org/#concept-tree-host-including-inclusive-ancestor
    pub fn is_host_inclusive_ancestor_of(&self, parent: &Node) -> bool {
        // TODO: Step 1.2
        self.is_ancestor_of(parent)
    }

    // https://dom.spec.whatwg.org/#concept-node-pre-insert
    pub fn pre_insert(
        node: Rc<Node>,
        parent: &Node,
        child: Option<Rc<Node>>,
    ) -> Fallible<Rc<Node>> {
        // Step 1
        Node::ensure_pre_insertion_validity(node.clone(), parent, child.clone())?;

        // Step 2-3
        let reference_child = match child {
            Some(child) if child == node => node.get_next_sibling(),
            _ => child,
        };

        // Step 4
        Node::insert(
            node.clone(),
            parent,
            reference_child,
            SuppressObserver::Unsuppressed,
        );
        Ok(node)
    }

    // https://dom.spec.whatwg.org/#concept-node-insert
    fn insert(
        node: Rc<Node>,
        parent: &Node,
        child: Option<Rc<Node>>,
        suppress_observers: SuppressObserver,
    ) {
        // TODO Step 5, 6, 7.5-7, 8-9

        // Step 1
        if node.node_type_id.is_document_fragment() {
            // Step 2-3
            if node.children().count() == 0 {
                return;
            }

            // Step 4
            node.children()
                .for_each(|child| node.remove(child, suppress_observers));

            // Step 7
            for node in node.children() {
                Node::adopt(node.as_ref(), parent.get_owner_doc().unwrap());
                parent.add_child(node, child.clone());
            }
        } else {
            // Step 7
            if let Some(document) = parent.get_owner_doc() {
                Node::adopt(node.as_ref(), document);
            }
            parent.add_child(node, child.clone());
        };
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
            },
            NodeTypeId::DocumentType => {
                if !parent.node_type_id.is_document() {
                    return Err(Error::HierarchyRequest);
                }
            },
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
                                if child
                                    .following_siblings()
                                    .any(|sibling| sibling.node_type_id.is_document_type())
                                {
                                    return Err(Error::HierarchyRequest);
                                }
                            }
                        },
                        _ => return Err(Error::HierarchyRequest),
                    }
                },
                NodeTypeId::Element(_) => {},
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
                                .any(|sibling| sibling.node_type_id.is_element())
                            {
                                return Err(Error::HierarchyRequest);
                            }
                        },
                        None => {
                            if parent
                                .children()
                                .any(|child| child.node_type_id.is_element())
                            {
                                return Err(Error::HierarchyRequest);
                            }
                        },
                    }
                },
                _ => (),
            }
        }

        Ok(())
    }

    // https://dom.spec.whatwg.org/#concept-node-adopt
    pub fn adopt(node: &Node, document: Rc<Document>) {
        // TODO Step 3.1.2, 3.2

        // Step 1
        let old_document = node.get_owner_doc();

        // Step 2
        if let Some(parent) = node.get_parent_node() {
            parent.remove(Rc::new(node.clone()), SuppressObserver::Unsuppressed);
        }

        // Step 3
        if let Some(old_document) = old_document {
            // Step 3.1
            if std::ptr::eq(old_document.as_ref(), document.as_ref()) {
                for descendant in node.traverse_preorder() {
                    descendant.set_owner_doc(document.clone())
                }
            }

            // Step 3.3
            for descendant in node.traverse_preorder() {
                vtable_for(&descendant).adopting_steps(old_document.clone());
            }
        }
    }

    pub fn is_parent_of(&self, child: Rc<Node>) -> bool {
        child
            .get_parent_node()
            .map_or(false, |parent| parent.as_ref() == self)
    }

    // https://dom.spec.whatwg.org/#concept-node-pre-remove
    fn pre_remove(child: Rc<Node>, parent: &Node) -> Fallible<Rc<Node>> {
        // Step 1
        if child.get_parent_node().unwrap().as_ref() != parent {
            return Err(Error::NotFound);
        }

        // Step 2
        parent.remove(child.clone(), SuppressObserver::Unsuppressed);

        // Step 3
        Ok(child)
    }

    // https://dom.spec.whatwg.org/#concept-node-remove
    fn remove(&self, child: Rc<Node>, _suppress_observers: SuppressObserver) {
        assert!(child.get_parent_node().is_some());

        // Step 9-11
        match child.get_prev_sibling() {
            Some(prev_sibling) => {
                prev_sibling
                    .next_sibling
                    .replace(Some(Rc::downgrade(&child.get_next_sibling().unwrap())));
            },
            None => {
                self.first_child.replace(child.get_next_sibling());
            },
        }

        match child.get_next_sibling() {
            Some(next_sibling) => {
                next_sibling
                    .prev_sibling
                    .replace(Some(Rc::downgrade(&child.get_prev_sibling().unwrap())));
            },
            None => {
                self.last_child.replace(child.get_prev_sibling());
            },
        }
        child.prev_sibling.replace(None);
        child.next_sibling.replace(None);
        child.parent_node.replace(None);

        // Step 15
        vtable_for(&child).unbind_from_tree();
    }

    pub fn following_siblings(&self) -> impl Iterator<Item = Rc<Node>> {
        SimpleNodeIterator {
            current: self.get_next_sibling(),
            next_node: |n: &Rc<Node>| n.get_next_sibling(),
        }
    }

    pub fn preceding_siblings(&self) -> impl Iterator<Item = Rc<Node>> {
        SimpleNodeIterator {
            current: self.get_prev_sibling(),
            next_node: |n: &Rc<Node>| n.get_prev_sibling(),
        }
    }

    pub fn children(&self) -> impl Iterator<Item = Rc<Node>> {
        SimpleNodeIterator {
            current: self.first_child.borrow().deref().clone(),
            next_node: |n: &Rc<Node>| n.get_next_sibling(),
        }
    }
}

impl VirtualMethods for Node {
    fn super_type(&self) -> Option<&dyn VirtualMethods> {
        None
    }
}

/// suppress observers flag
/// <https://dom.spec.whatwg.org/#concept-node-insert>
/// <https://dom.spec.whatwg.org/#concept-node-remove>
#[derive(Clone, Copy)]
enum SuppressObserver {
    Suppressed,
    Unsuppressed,
}

pub struct SimpleNodeIterator<T, I>
where
    I: Fn(&T) -> Option<T>,
{
    current: Option<T>,
    next_node: I,
}

impl<T, I> Iterator for SimpleNodeIterator<T, I>
where
    I: Fn(&T) -> Option<T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current.take();
        self.current = current.as_ref().and_then(|c| (self.next_node)(c));
        current
    }
}

pub fn document_from_node<T: DerivedFrom<Node>>(derived: &T) -> Rc<Document> {
    derived.upcast().get_owner_doc().unwrap()
}

pub struct TreeIterator {
    current: Option<Rc<Node>>,
    depth: usize,
}

impl TreeIterator {
    fn new(root: Rc<Node>) -> TreeIterator {
        TreeIterator {
            current: Some(root),
            depth: 0,
        }
    }

    pub fn next_skipping_children(&mut self) -> Option<Rc<Node>> {
        let current = self.current.take()?;

        self.next_skipping_children_impl(current)
    }

    fn next_skipping_children_impl(&mut self, current: Rc<Node>) -> Option<Rc<Node>> {
        let iter = current.ancestors();

        for ancestor in iter {
            if self.depth == 0 {
                break;
            }
            if let Some(next_sibling) = ancestor.get_next_sibling() {
                self.current = Some(next_sibling);
                return Some(current);
            }
            self.depth -= 1;
        }
        debug_assert_eq!(self.depth, 0);
        self.current = None;
        Some(current)
    }
}

impl Iterator for TreeIterator {
    type Item = Rc<Node>;

    // https://dom.spec.whatwg.org/#concept-tree-order
    fn next(&mut self) -> Option<Rc<Node>> {
        let current = self.current.take()?;

        if let Some(first_child) = current.get_first_child() {
            self.current = Some(first_child);
            self.depth += 1;
            return Some(current);
        };

        self.next_skipping_children_impl(current)
    }
}
