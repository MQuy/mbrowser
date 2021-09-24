use std::cell::RefCell;
use std::path::Path;
use std::rc::{Rc, Weak};
use std::{env, fs};

use css::media_queries::media_list::MediaList;
use css::stylesheets::origin::Origin;
use css::stylesheets::stylesheet::Stylesheet;
use css::stylist::Stylist;
use dom::global_scope::NodeRef;
use dom::window::Window;
use selectors::context::QuirksMode;

use crate::applicable_declaration_block::ApplicableDeclarationBlock;
use crate::rule_colectors::collect_rules;

/// https://chromium.googlesource.com/chromium/blink/+/refs/heads/main/Source/core/css/html.css
/// https://trac.webkit.org/browser/trunk/Source/WebCore/css/html.css
// https://searchfox.org/mozilla-central/source/layout/style/res/html.css
#[derive(Debug)]
pub struct StyleTree {
	root: Rc<StyleTreeNode>,
	window: Rc<Window>,
	stylist: RefCell<Stylist>,
}

impl StyleTree {
	pub fn new(dom_node: NodeRef, quirks_mode: QuirksMode) -> Self {
		Self {
			window: dom_node.window().unwrap(),
			stylist: RefCell::new(Stylist::new(quirks_mode)),
			root: Rc::new(StyleTreeNode::new(dom_node, None)),
		}
	}

	pub fn stylist(&self) -> &Stylist {
		unsafe { self.stylist.as_ptr().as_ref().unwrap() }
	}

	pub fn import_user_agent(&self) {
		let component_path = env::current_dir().unwrap();
		let user_agent_path = Path::join(&component_path, "./src/html.css");
		let content = fs::read_to_string(user_agent_path).unwrap();
		let stylesheet = Stylesheet::from_str(
			&content,
			Origin::UserAgent,
			Rc::new(MediaList::empty()),
			Some(self.window.error_reporter()),
			selectors::context::QuirksMode::NoQuirks,
			0,
		);
		self.stylist
			.borrow_mut()
			.add_stylesheet(&stylesheet, Origin::UserAgent);
	}

	pub fn add_stylesheet(&self, stylesheet: &Stylesheet) {
		self.stylist
			.borrow_mut()
			.add_stylesheet(&stylesheet, Origin::Author);
	}

	pub fn match_rules(&self) {
		self.match_rule_for_node(self.root.clone());
	}

	fn match_rule_for_node(&self, style_node: Rc<StyleTreeNode>) {
		let rules = collect_rules(style_node.dom_node.clone(), self.stylist());
		*style_node.rules.borrow_mut() = rules;

		let mut dom_child = style_node.dom_node.first_child();
		loop {
			let noderef_child = if let Some(noderef_child) = &dom_child {
				noderef_child
			} else {
				break;
			};
			let style_child = Rc::new(StyleTreeNode::new(
				noderef_child.clone(),
				Some(style_node.clone()),
			));
			style_node.append_child(style_child.clone());
			if noderef_child.node_type_id().is_element() {
				self.match_rule_for_node(style_child.clone());
			}

			dom_child = dom_child.map(|d| d.next_sibling()).flatten();
		}
	}
}

#[derive(Debug)]
pub struct StyleTreeNode {
	pub dom_node: NodeRef,
	pub rules: RefCell<Vec<ApplicableDeclarationBlock>>,
	pub parent_node: RefCell<Option<Weak<StyleTreeNode>>>,
	pub first_child: RefCell<Option<Rc<StyleTreeNode>>>,
	pub last_child: RefCell<Option<Rc<StyleTreeNode>>>,
	pub next_sibling: RefCell<Option<Weak<StyleTreeNode>>>,
	pub prev_sibling: RefCell<Option<Weak<StyleTreeNode>>>,
}

impl StyleTreeNode {
	pub fn new(node: NodeRef, parent_node: Option<Rc<StyleTreeNode>>) -> Self {
		StyleTreeNode {
			dom_node: node.clone(),
			rules: Default::default(),
			parent_node: RefCell::new(parent_node.map(|n| Rc::downgrade(&n))),
			first_child: Default::default(),
			last_child: Default::default(),
			next_sibling: Default::default(),
			prev_sibling: Default::default(),
		}
	}

	pub fn append_child(&self, node: Rc<StyleTreeNode>) {
		if let Some(last_child) = self.last_child.borrow().as_ref() {
			last_child.next_sibling.replace(Some(Rc::downgrade(&node)));
			node.prev_sibling.replace(Some(Rc::downgrade(&last_child)));
		} else {
			self.first_child.replace(Some(node.clone()));
		}

		self.last_child.replace(Some(node.clone()));
	}
}
