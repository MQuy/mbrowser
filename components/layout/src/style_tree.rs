use std::cell::RefCell;
use std::collections::HashMap;
use std::path::Path;
use std::rc::{Rc, Weak};
use std::{env, fs};

use css::computed_values::{ComputedValues, PropertyCascade, StyleContext};
use css::media_queries::media_list::MediaList;
use css::properties::declaration::{PropertyDeclaration, WideKeywordDeclaration};
use css::properties::longhand_id::{LonghandId, LonghandIdPhaseIterator, PhaseOrder};
use css::properties::property_id::CSSWideKeyword;
use css::stylesheets::origin::Origin;
use css::stylesheets::stylesheet::Stylesheet;
use css::stylist::Stylist;
use dom::global_scope::{GlobalScope, NodeRef};
use dom::window::Window;
use selectors::context::QuirksMode;

use crate::applicable_declaration_block::{ApplicableDeclarationBlock, StyleSource};
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

	pub fn root(&self) -> Rc<StyleTreeNode> {
		self.root.clone()
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
		while let Some(noderef_child) = &dom_child {
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

	pub fn first_child(&self) -> Option<Rc<StyleTreeNode>> {
		self.first_child.borrow().clone()
	}

	pub fn parent(&self) -> Option<Rc<StyleTreeNode>> {
		match self.parent_node.borrow().as_ref() {
			Some(node) => node.upgrade(),
			_ => None,
		}
	}

	pub fn next_sibling(&self) -> Option<Rc<StyleTreeNode>> {
		match self.next_sibling.borrow().as_ref() {
			Some(node) => node.upgrade(),
			_ => None,
		}
	}
}

pub fn cascade(style_node: Rc<StyleTreeNode>, parent_style: &ComputedValues) {
	let mut author_data: HashMap<LonghandId, PropertyCascade> = HashMap::new();
	let mut useragent_data: HashMap<LonghandId, PropertyCascade> = HashMap::new();
	let rules = style_node.rules.borrow();
	for declaration in rules.iter() {
		let block = match &declaration.source {
			StyleSource::StyleRule(style) => &style.block,
			StyleSource::DeclarationBlock(block) => block,
		};
		for (importance, property) in block.properties() {
			match declaration.origin {
				Origin::UserAgent => {
					cascade_in_origin(
						&mut useragent_data,
						property,
						importance,
						declaration.specificity,
					);
				},
				Origin::Author => {
					cascade_in_origin(
						&mut author_data,
						property,
						importance,
						declaration.specificity,
					);
				},
			}
		}
	}
	let mut computed_values = GlobalScope::get_or_init_computed_values(style_node.dom_node.id());
	let mut context = StyleContext {
		parent_style,
		author_data,
		useragent_data,
		computed_values: &mut computed_values,
	};
	apply_properties(LonghandId::ids(PhaseOrder::Early), &mut context);
	apply_properties(LonghandId::ids(PhaseOrder::Other), &mut context);

	let mut child = style_node.first_child.borrow().as_ref().map(|n| n.clone());
	while let Some(noderef) = child {
		cascade(noderef.clone(), computed_values);
		child = if let Some(child) = noderef.next_sibling.borrow().as_ref() {
			child.upgrade()
		} else {
			None
		};
	}
}

fn cascade_in_origin<'a, 'b>(
	cascade_data: &'a mut HashMap<LonghandId, PropertyCascade<'b>>,
	property: &'b PropertyDeclaration,
	importance: bool,
	specificity: u32,
) {
	if let Some(cascade) = cascade_data.get(&property.longhand_id()) {
		if (cascade.importance && !importance) || (cascade.specificity > specificity) {
			return;
		}
	}
	cascade_data.insert(
		property.longhand_id(),
		PropertyCascade {
			specificity,
			importance,
			property,
		},
	);
}

fn get_declaration_from_useragent<'a>(
	cascade_data: &HashMap<LonghandId, PropertyCascade<'a>>,
	longhand_id: &LonghandId,
	unset: &'a PropertyDeclaration,
) -> Option<&'a PropertyDeclaration> {
	let useragent_property = cascade_data
		.get(longhand_id)
		.map(|cascade| cascade.property);
	if let Some(property) = useragent_property {
		Some(match property {
			PropertyDeclaration::CSSWideKeyword(WideKeywordDeclaration { keyword, .. })
				if *keyword == CSSWideKeyword::Revert =>
			{
				unset
			},
			value => value,
		})
	} else {
		None
	}
}

fn apply_properties<'a>(longhands_iter: LonghandIdPhaseIterator, context: &'a mut StyleContext) {
	for longhand_id in longhands_iter {
		let unset = PropertyDeclaration::CSSWideKeyword(WideKeywordDeclaration {
			id: longhand_id,
			keyword: CSSWideKeyword::Unset,
		});
		let author_property = context
			.author_data
			.get(&longhand_id)
			.map(|cascade| cascade.property);
		let declaration = if let Some(property) = author_property {
			match property {
				PropertyDeclaration::CSSWideKeyword(WideKeywordDeclaration { keyword, .. })
					if *keyword == CSSWideKeyword::Revert =>
				{
					get_declaration_from_useragent(&context.useragent_data, &longhand_id, &unset)
				},
				value => Some(value),
			}
		} else {
			get_declaration_from_useragent(&context.useragent_data, &longhand_id, &unset)
		};
		longhand_id.cascade(declaration, context)
	}
}
