use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

use common::{not_reached, not_supported};
use css::computed_values::{ComputedValues, PropertyCascade, StyleContext};
use css::media_queries::media_list::MediaList;
use css::properties::declaration::{PropertyDeclaration, WideKeywordDeclaration};
use css::properties::longhand_id::{LonghandId, LonghandIdPhaseIterator, PhaseOrder};
use css::properties::longhands;
use css::properties::longhands::display::{DisplayBasic, DisplayInside, DisplayOutside};
use css::properties::property_id::CSSWideKeyword;
use css::stylesheets::origin::Origin;
use css::stylesheets::stylesheet::Stylesheet;
use css::stylist::Stylist;
use dom::global_scope::{GlobalScope, NodeRef};
use dom::node::SimpleNodeIterator;
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
		let content = include_str!("./html.css");
		let stylesheet = Stylesheet::from_str(
			&content,
			Origin::UserAgent,
			Rc::new(MediaList::empty()),
			Some(self.window.error_reporter()),
			selectors::context::QuirksMode::NoQuirks,
			0,
		);
		self.stylist.borrow_mut().add_stylesheet(&stylesheet, Origin::UserAgent);
	}

	pub fn add_stylesheet(&self, stylesheet: &Stylesheet) {
		self.stylist.borrow_mut().add_stylesheet(&stylesheet, Origin::Author);
	}

	pub fn match_rules(&self) {
		self.match_rule_for_node(self.root.clone());
	}

	fn match_rule_for_node(&self, style_node: Rc<StyleTreeNode>) {
		let rules = collect_rules(style_node.dom_node.clone(), self.stylist());
		*style_node.rules.borrow_mut() = rules;

		let mut dom_child = style_node.dom_node.first_child();
		while let Some(noderef_child) = &dom_child {
			let style_child = Rc::new(StyleTreeNode::new(noderef_child.clone(), Some(style_node.clone())));

			style_node.append_child(style_child.clone());
			if noderef_child.node_type_id().is_element() {
				self.match_rule_for_node(style_child.clone());
			}

			dom_child = dom_child.map(|d| d.next_sibling()).flatten();
		}
	}

	pub fn cascade(&self) {
		self.cascade_node(self.root.clone(), &ComputedValues::default())
	}

	fn cascade_node(&self, style_node: Rc<StyleTreeNode>, parent_style: &ComputedValues) {
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
						cascade_in_origin(&mut useragent_data, property, importance, declaration.specificity);
					},
					Origin::Author => {
						cascade_in_origin(&mut author_data, property, importance, declaration.specificity);
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
			self.cascade_node(noderef.clone(), computed_values);
			child = if let Some(child) = noderef.next_sibling.borrow().as_ref() {
				Some(child.clone())
			} else {
				None
			};
		}
	}

	pub fn log(src: Rc<StyleTreeNode>, depth: usize) {
		let indent: String = std::iter::repeat("  ").take(depth).collect();
		println!("{}{:?}", indent, src.dom_node.node_type_id());
		let iter = SimpleNodeIterator::new(src.first_child(), |n: &Rc<StyleTreeNode>| n.next_sibling());
		for child in iter {
			StyleTree::log(child, depth + 1);
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
	pub next_sibling: RefCell<Option<Rc<StyleTreeNode>>>,
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
			last_child.next_sibling.replace(Some(node.clone()));
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
			Some(node) => Some(node.clone()),
			_ => None,
		}
	}

	pub fn get_visible_children_iter(&self) -> impl Iterator<Item = Rc<StyleTreeNode>> {
		fn adjust_current_node(node: Option<Rc<StyleTreeNode>>) -> Option<Rc<StyleTreeNode>> {
			let mut current = node;
			while let Some(ref style_node) = current {
				let computed_values = GlobalScope::get_or_init_computed_values(style_node.dom_node.id());
				match computed_values.get_display() {
					longhands::display::Display::Box(value) if *value == longhands::display::DisplayBox::None => {
						current = style_node.next_sibling();
						continue;
					},
					_ => break,
				}
			}
			current
		}
		SimpleNodeIterator::new(adjust_current_node(self.first_child()), |n: &Rc<StyleTreeNode>| {
			adjust_current_node(n.next_sibling())
		})
	}

	pub fn is_contain_all_inline_children(&self) -> bool {
		let children_iter = self.get_visible_children_iter();
		for child in children_iter {
			let (outside, _) = child.get_display();
			if outside != DisplayOutside::Inline {
				return false;
			}
		}
		true
	}

	pub fn get_display(&self) -> (DisplayOutside, DisplayInside) {
		if self.dom_node.node_type_id().is_element() {
			let computed_values = GlobalScope::get_or_init_computed_values(self.dom_node.id());
			match computed_values.get_display() {
				longhands::display::Display::Basic(DisplayBasic { outside, inside }) => (
					outside.as_ref().map_or(DisplayOutside::Block, |v| v.clone()),
					inside.as_ref().clone().map_or(DisplayInside::Flow, |v| v.clone()),
				),
				longhands::display::Display::Box(value) => match value {
					longhands::display::DisplayBox::Contents => not_supported!(),
					longhands::display::DisplayBox::None => not_reached!(),
				},
				longhands::display::Display::Legacy(legacy)
					if *legacy == longhands::display::DisplayLegacy::InlineBlock =>
				{
					(DisplayOutside::Inline, DisplayInside::FlowRoot)
				},
				_ => not_supported!(),
			}
		} else {
			(DisplayOutside::Inline, DisplayInside::Flow)
		}
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
	let useragent_property = cascade_data.get(longhand_id).map(|cascade| cascade.property);
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
		let author_property = context.author_data.get(&longhand_id).map(|cascade| cascade.property);
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
