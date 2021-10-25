use core::panic;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use css::computed_values::ComputedValues;
use css::element_state::ElementState;
use css::properties::declaration_block::PropertyDeclarationBlock;
use html5ever::{LocalName, Namespace};
use selectors::matching::ElementSelectorFlags;

use crate::attr::Attr;
use crate::element::Element;
use crate::inheritance::downcast;
use crate::node::Node;
use crate::nodetype::NodeTypeId;
use crate::window::Window;

#[derive(Clone, Debug)]
pub struct NodeRef(pub Rc<Node>);

impl NodeRef {
	pub fn parent(&self) -> Option<NodeRef> {
		self.0.parent_node().map(|v| NodeRef(v))
	}

	pub fn prev_sibling(&self) -> Option<NodeRef> {
		self.0.prev_sibling().map(|v| NodeRef(v))
	}

	pub fn next_sibling(&self) -> Option<NodeRef> {
		self.0.next_sibling().map(|v| NodeRef(v))
	}

	pub fn first_child(&self) -> Option<NodeRef> {
		self.0.first_child().map(|v| NodeRef(v))
	}

	pub fn last_child(&self) -> Option<NodeRef> {
		self.0.last_child().map(|v| NodeRef(v))
	}

	pub fn namespace(&self) -> Namespace {
		assert!(self.0.node_type_id().is_element());
		downcast::<Node, Element>(self.0.clone())
			.namespace()
			.clone()
	}

	pub fn local_name(&self) -> LocalName {
		assert!(self.0.node_type_id().is_element());
		downcast::<Node, Element>(self.0.clone())
			.local_name()
			.clone()
	}

	pub fn get_attribute(&self, namespace: &Namespace, local_name: &LocalName) -> Option<Rc<Attr>> {
		assert!(self.0.node_type_id().is_element());
		downcast::<Node, Element>(self.0.clone()).get_attribute(namespace, local_name)
	}

	pub fn attrs(&self) -> RefCell<Vec<Rc<Attr>>> {
		assert!(self.0.node_type_id().is_element());
		downcast::<Node, Element>(self.0.clone()).attrs()
	}

	pub fn state(&self) -> ElementState {
		assert!(self.0.node_type_id().is_element());
		downcast::<Node, Element>(self.0.clone()).state()
	}

	pub fn has_attribute(&self, local_name: &LocalName) -> bool {
		assert!(self.0.node_type_id().is_element());
		downcast::<Node, Element>(self.0.clone()).has_attribute(local_name)
	}

	pub fn node_type_id(&self) -> NodeTypeId {
		self.0.node_type_id()
	}

	pub fn children(&self) -> impl Iterator<Item = Rc<Node>> {
		self.0.children()
	}

	pub fn parent_node(&self) -> Option<Rc<Node>> {
		self.0.parent_node()
	}

	pub fn insert_selector_flags(&self, flags: ElementSelectorFlags) {
		assert!(self.0.node_type_id().is_element());
		downcast::<Node, Element>(self.0.clone()).insert_selector_flags(flags)
	}

	pub fn style_attribute(&self) -> RefCell<Option<PropertyDeclarationBlock>> {
		assert!(self.0.node_type_id().is_element());
		downcast::<Node, Element>(self.0.clone())
			.style_attribute()
			.clone()
	}

	pub fn window(&self) -> Option<Rc<Window>> {
		match self.0.owner_doc() {
			Some(document) => document.window(),
			None => None,
		}
	}

	pub fn id(&self) -> u64 {
		self.0.id()
	}
}

pub struct GlobalScope {
	counted: u64,
	nodes: Option<HashMap<u64, Rc<Node>>>,
	computed_values: Option<HashMap<u64, ComputedValues>>,
}

impl GlobalScope {
	fn init_computed_values<'a>(id: u64) -> &'a mut ComputedValues {
		unsafe {
			if let Some(values) = &mut SCOPE.computed_values {
				values.insert(id, ComputedValues::default());
				return values.get_mut(&id).unwrap();
			}
			unreachable!()
		}
	}

	pub fn get_or_init_computed_values<'a>(id: u64) -> &'a mut ComputedValues {
		unsafe {
			if SCOPE.computed_values.is_none() {
				SCOPE.computed_values = Some(HashMap::new());
			}
			if let Some(values) = &mut SCOPE.computed_values {
				if let Some(value) = values.get_mut(&id) {
					value
				} else {
					GlobalScope::init_computed_values(id)
				}
			} else {
				unreachable!()
			}
		}
	}

	pub fn get_node(id: u64) -> Rc<Node> {
		unsafe {
			match &SCOPE.nodes {
				Some(nodes) => nodes.get(&id).unwrap().clone(),
				None => panic!(),
			}
		}
	}

	pub fn add_node(node: Rc<Node>) -> Rc<Node> {
		unsafe {
			if SCOPE.nodes.is_none() {
				SCOPE.nodes = Some(HashMap::new());
			}
			if let Some(nodes) = &mut SCOPE.nodes {
				nodes.insert(node.id(), node.clone());
			}
			node
		}
	}

	pub fn get_next_id() -> u64 {
		unsafe {
			SCOPE.counted += 1;
			SCOPE.counted
		}
	}
}

static mut SCOPE: GlobalScope = GlobalScope {
	counted: 0,
	nodes: None,
	computed_values: None,
};
