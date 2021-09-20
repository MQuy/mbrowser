use core::panic;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use css::element_state::ElementState;
use css::properties::declaration_block::PropertyDeclarationBlock;
use html5ever::{LocalName, Namespace};
use selectors::matching::ElementSelectorFlags;

use crate::attr::Attr;
use crate::element::Element;
use crate::inheritance::downcast;
use crate::node::Node;
use crate::nodetype::NodeTypeId;

#[derive(Clone, Debug)]
pub struct NodeRef(pub Rc<Node>);

impl NodeRef {
	pub fn parent_element(&self) -> Option<NodeRef> {
		self.0.get_parent_node().map(|v| NodeRef(v))
	}

	pub fn prev_sibling_element(&self) -> Option<NodeRef> {
		self.0.get_prev_sibling().map(|v| NodeRef(v))
	}

	pub fn next_sibling_element(&self) -> Option<NodeRef> {
		self.0.get_next_sibling().map(|v| NodeRef(v))
	}

	pub fn get_namespace(&self) -> Namespace {
		assert!(self.0.get_node_type_id().is_element());
		downcast::<Node, Element>(self.0.clone())
			.namespace()
			.clone()
	}

	pub fn get_local_name(&self) -> LocalName {
		assert!(self.0.get_node_type_id().is_element());
		downcast::<Node, Element>(self.0.clone())
			.local_name()
			.clone()
	}

	pub fn get_attribute(&self, namespace: &Namespace, local_name: &LocalName) -> Option<Rc<Attr>> {
		assert!(self.0.get_node_type_id().is_element());
		downcast::<Node, Element>(self.0.clone()).get_attribute(namespace, local_name)
	}

	pub fn get_attrs(&self) -> RefCell<Vec<Rc<Attr>>> {
		assert!(self.0.get_node_type_id().is_element());
		downcast::<Node, Element>(self.0.clone()).get_attrs()
	}

	pub fn state(&self) -> ElementState {
		assert!(self.0.get_node_type_id().is_element());
		downcast::<Node, Element>(self.0.clone()).get_state()
	}

	pub fn has_attribute(&self, local_name: &LocalName) -> bool {
		assert!(self.0.get_node_type_id().is_element());
		downcast::<Node, Element>(self.0.clone()).has_attribute(local_name)
	}

	pub fn get_node_type_id(&self) -> NodeTypeId {
		self.0.get_node_type_id()
	}

	pub fn children(&self) -> impl Iterator<Item = Rc<Node>> {
		self.0.children()
	}

	pub fn get_parent_node(&self) -> Option<Rc<Node>> {
		self.0.get_parent_node()
	}

	pub fn insert_selector_flags(&self, flags: ElementSelectorFlags) {
		assert!(self.0.get_node_type_id().is_element());
		downcast::<Node, Element>(self.0.clone()).insert_selector_flags(flags)
	}

	pub fn get_style_attribute(&self) -> RefCell<Option<PropertyDeclarationBlock>> {
		assert!(self.0.get_node_type_id().is_element());
		downcast::<Node, Element>(self.0.clone())
			.get_style_attribute()
			.clone()
	}
}

struct GlobalScope {
	counted: u64,
	nodes: Option<HashMap<u64, Rc<Node>>>,
}

impl GlobalScope {}

static mut SCOPE: GlobalScope = GlobalScope {
	counted: 0,
	nodes: None,
};

pub fn add_to_global_scope(node: Rc<Node>) -> Rc<Node> {
	unsafe {
		if SCOPE.nodes.is_none() {
			SCOPE.nodes = Some(HashMap::new());
		}
		if let Some(nodes) = &mut SCOPE.nodes {
			nodes.insert(node.get_id(), node.clone());
		}
		node
	}
}

pub fn get_from_global_scope(id: u64) -> Rc<Node> {
	unsafe {
		match &SCOPE.nodes {
			Some(nodes) => nodes.get(&id).unwrap().clone(),
			None => panic!(),
		}
	}
}

pub fn get_next_id() -> u64 {
	unsafe {
		SCOPE.counted += 1;
		SCOPE.counted
	}
}
