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

/// https://chromium.googlesource.com/chromium/blink/+/refs/heads/main/Source/core/css/html.css
/// https://trac.webkit.org/browser/trunk/Source/WebCore/css/html.css
// https://searchfox.org/mozilla-central/source/layout/style/res/html.css
#[derive(Debug)]
pub struct StyleTree {
	root: StyleTreeNode,
	window: Rc<Window>,
	stylist: RefCell<Stylist>,
}

impl StyleTree {
	pub fn new(dom_node: NodeRef, quirks_mode: QuirksMode) -> Self {
		Self {
			window: dom_node.window().unwrap(),
			stylist: RefCell::new(Stylist::new(quirks_mode)),
			root: StyleTreeNode::new(dom_node),
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
}

#[derive(Debug)]
pub struct StyleTreeNode {
	dom_node: NodeRef,
	rules: Vec<ApplicableDeclarationBlock>,
	parent_node: RefCell<Option<Weak<StyleTreeNode>>>,
	first_child: RefCell<Option<Rc<StyleTreeNode>>>,
	last_child: RefCell<Option<Rc<StyleTreeNode>>>,
	next_sibling: RefCell<Option<Weak<StyleTreeNode>>>,
	prev_sibling: RefCell<Option<Weak<StyleTreeNode>>>,
}

impl StyleTreeNode {
	pub fn new(node: NodeRef) -> Self {
		StyleTreeNode {
			dom_node: node.clone(),
			rules: Default::default(),
			parent_node: Default::default(),
			first_child: Default::default(),
			last_child: Default::default(),
			next_sibling: Default::default(),
			prev_sibling: Default::default(),
		}
	}
}
