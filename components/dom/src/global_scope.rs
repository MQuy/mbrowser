use core::panic;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;

use css::computed_values::ComputedValues;
use css::element_state::ElementState;
use css::properties::declaration_block::PropertyDeclarationBlock;
use html5ever::{LocalName, Namespace};
use once_cell::sync::Lazy;
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
}

impl Deref for NodeRef {
	type Target = Node;

	fn deref(&self) -> &Node {
		self.0.deref()
	}
}

pub struct GlobalScope {
	number_of_doms: u64,
	doms: HashMap<u64, Rc<Node>>,
	computed_values: HashMap<u64, ComputedValues>,
}

impl GlobalScope {
	pub fn get_or_init_computed_values<'a>(id: u64) -> &'a mut ComputedValues {
		unsafe {
			if let Some(value) = SCOPE.computed_values.get_mut(&id) {
				value
			} else {
				SCOPE.computed_values.insert(id, ComputedValues::default());
				SCOPE.computed_values.get_mut(&id).unwrap()
			}
		}
	}

	pub fn get_node(id: u64) -> Rc<Node> {
		unsafe { SCOPE.doms.get(&id).unwrap().clone() }
	}

	pub fn add_node(node: Rc<Node>) -> Rc<Node> {
		unsafe {
			SCOPE.doms.insert(node.id(), node.clone());
			node
		}
	}

	pub fn get_next_id() -> u64 {
		unsafe {
			SCOPE.number_of_doms += 1;
			SCOPE.number_of_doms
		}
	}

	pub fn clear() {
		unsafe {
			SCOPE.number_of_doms = 0;
			SCOPE.doms.clear();
			SCOPE.computed_values.clear();
		}
	}
}

static mut SCOPE: Lazy<GlobalScope> = Lazy::new(|| GlobalScope {
	number_of_doms: 0,
	doms: HashMap::new(),
	computed_values: HashMap::new(),
});
