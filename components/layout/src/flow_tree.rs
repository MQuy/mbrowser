use std::cell::RefCell;
use std::rc::{Rc, Weak};

use common::{not_reached, not_supported};
use css::properties::longhands;
use css::properties::longhands::display::{DisplayBasic, DisplayInside, DisplayOutside};
use dom::global_scope::{GlobalScope, NodeRef};
use dom::node::SimpleNodeIterator;

use crate::style_tree::{StyleTree, StyleTreeNode};

pub struct BaseBox {
	pub dom_node: NodeRef,
	pub formatting_context: RefCell<Rc<FormattingContext>>,
	pub children: RefCell<Vec<Rc<VisualBox>>>,
	pub parent: RefCell<Option<Weak<VisualBox>>>,
}

impl BaseBox {
	pub fn new(dom_node: NodeRef, formatting_context: Rc<FormattingContext>) -> Self {
		BaseBox {
			dom_node,
			parent: RefCell::new(Default::default()),
			formatting_context: RefCell::new(formatting_context),
			children: RefCell::new(Default::default()),
		}
	}

	pub fn add_child(&self, child: Rc<VisualBox>) {
		self.children.borrow_mut().push(child)
	}

	pub fn formatting_context(&self) -> Rc<FormattingContext> {
		self.formatting_context.borrow().clone()
	}

	pub fn set_formatting_context(&self, formatting_context: Rc<FormattingContext>) {
		self.formatting_context.replace(formatting_context);
	}

	pub fn formatting_context_type(&self) -> FormattingContextType {
		self.formatting_context.borrow().formatting_context_type
	}

	pub fn get_last_child(&self) -> Option<Rc<VisualBox>> {
		self.children.borrow().last().map(|value| value.clone())
	}

	pub fn parent(&self) -> Option<Rc<VisualBox>> {
		match self.parent.borrow().as_ref() {
			Some(value) => value.upgrade(),
			None => None,
		}
	}

	pub fn set_parent(&self, value: Option<Rc<VisualBox>>) {
		self.parent
			.replace(value.as_ref().map(|v| Rc::downgrade(v)));
	}
}

/// https://www.w3.org/TR/CSS22/visuren.html#block-boxes
pub struct BlockLevelBox {
	base: BaseBox,
}

impl BlockLevelBox {
	pub fn new(dom_node: NodeRef, formatting_context: Rc<FormattingContext>) -> Self {
		BlockLevelBox {
			base: BaseBox::new(dom_node, formatting_context),
		}
	}

	pub fn add_child(&self, child: Rc<VisualBox>) {
		self.base.add_child(child)
	}

	pub fn formatting_context(&self) -> Rc<FormattingContext> {
		self.base.formatting_context()
	}

	pub fn set_formatting_context(&self, formatting_context: Rc<FormattingContext>) {
		self.base.set_formatting_context(formatting_context);
	}

	pub fn formatting_context_type(&self) -> FormattingContextType {
		self.base.formatting_context_type()
	}

	pub fn get_last_child(&self) -> Option<Rc<VisualBox>> {
		self.base.get_last_child()
	}

	pub fn parent(&self) -> Option<Rc<VisualBox>> {
		self.base.parent()
	}

	pub fn set_parent(&self, value: Option<Rc<VisualBox>>) {
		self.base.set_parent(value);
	}
}

/// https://www.w3.org/TR/CSS22/visuren.html#inline-boxes
pub struct InlineLevelBox {
	base: BaseBox,
}

impl InlineLevelBox {
	pub fn new(dom_node: NodeRef, formatting_context: Rc<FormattingContext>) -> Self {
		InlineLevelBox {
			base: BaseBox::new(dom_node, formatting_context),
		}
	}

	pub fn add_child(&self, child: Rc<VisualBox>) {
		self.base.add_child(child)
	}

	pub fn formatting_context(&self) -> Rc<FormattingContext> {
		self.base.formatting_context()
	}

	pub fn set_formatting_context(&self, formatting_context: Rc<FormattingContext>) {
		self.base.set_formatting_context(formatting_context);
	}

	pub fn formatting_context_type(&self) -> FormattingContextType {
		self.base.formatting_context_type()
	}

	pub fn get_last_child(&self) -> Option<Rc<VisualBox>> {
		self.base.get_last_child()
	}

	pub fn parent(&self) -> Option<Rc<VisualBox>> {
		self.base.parent()
	}

	pub fn set_parent(&self, value: Option<Rc<VisualBox>>) {
		self.base.set_parent(value);
	}
}

pub struct AnonymousBox {
	boxes: RefCell<Vec<Rc<VisualBox>>>,
	parent: RefCell<Option<Rc<VisualBox>>>,
	formatting_context: RefCell<Rc<FormattingContext>>,
}

impl AnonymousBox {
	pub fn new(formatting_context: Rc<FormattingContext>) -> Self {
		AnonymousBox {
			parent: RefCell::new(Default::default()),
			boxes: RefCell::new(vec![]),
			formatting_context: RefCell::new(formatting_context),
		}
	}

	pub fn add_child(&self, child: Rc<VisualBox>) {
		self.boxes.borrow_mut().push(child)
	}

	pub fn formatting_context(&self) -> Rc<FormattingContext> {
		self.formatting_context.borrow().clone()
	}

	pub fn set_formatting_context(&self, formatting_context: Rc<FormattingContext>) {
		self.formatting_context.replace(formatting_context);
	}

	pub fn formatting_context_type(&self) -> FormattingContextType {
		self.formatting_context.borrow().formatting_context_type
	}

	pub fn get_last_child(&self) -> Option<Rc<VisualBox>> {
		self.boxes.borrow().last().map(|value| value.clone())
	}

	pub fn parent(&self) -> Option<Rc<VisualBox>> {
		self.parent.borrow().clone()
	}

	pub fn set_parent(&self, value: Option<Rc<VisualBox>>) {
		self.parent.replace(value);
	}
}

pub enum VisualBox {
	BlockBox(BlockLevelBox),
	InlineBox(InlineLevelBox),
	AnonymousBox(AnonymousBox),
}

impl VisualBox {
	pub fn new_with_formatting_context<F>(
		formatting_context_type: FormattingContextType,
		setup: F,
	) -> Rc<VisualBox>
	where
		F: FnOnce(Rc<FormattingContext>) -> Rc<VisualBox>,
	{
		let formatting_context = Rc::new(FormattingContext::new(formatting_context_type));
		let level_box = setup(formatting_context.clone());
		formatting_context.set_established_by(level_box.clone());
		level_box
	}

	pub fn formatting_context(&self) -> Rc<FormattingContext> {
		match self {
			VisualBox::BlockBox(block) => block.formatting_context(),
			VisualBox::InlineBox(inline) => inline.formatting_context(),
			VisualBox::AnonymousBox(value) => value.formatting_context(),
		}
	}

	pub fn set_formatting_context(&self, formatting_context: Rc<FormattingContext>) {
		match self {
			VisualBox::BlockBox(block) => block.set_formatting_context(formatting_context),
			VisualBox::InlineBox(inline) => inline.set_formatting_context(formatting_context),
			VisualBox::AnonymousBox(value) => value.set_formatting_context(formatting_context),
		}
	}

	pub fn formatting_context_type(&self) -> FormattingContextType {
		match self {
			VisualBox::BlockBox(block) => block.formatting_context_type(),
			VisualBox::InlineBox(inline) => inline.formatting_context_type(),
			VisualBox::AnonymousBox(value) => value.formatting_context_type(),
		}
	}

	pub fn append_child(source: Rc<VisualBox>, child: Rc<VisualBox>) {
		let child = match source.formatting_context_type() {
			FormattingContextType::BlockFormattingContext if child.is_inline_level() => {
				let last_child = source.get_last_child();
				if let Some(last_child) = last_child {
					if last_child.is_anonymous() {
						VisualBox::add_child(last_child, child);
						return;
					}
				}

				let anonymous_box = VisualBox::new_with_formatting_context(
					FormattingContextType::InlineFormattingContext,
					|formatting_context: Rc<FormattingContext>| {
						Rc::new(VisualBox::AnonymousBox(AnonymousBox::new(
							formatting_context,
						)))
					},
				);
				VisualBox::add_child(anonymous_box.clone(), child);
				anonymous_box
			},
			FormattingContextType::InlineFormattingContext if child.is_block_level() => {
				not_reached!()
			},
			_ => child,
		};
		VisualBox::add_child(source, child);
	}

	pub fn add_child(source: Rc<VisualBox>, child: Rc<VisualBox>) {
		match source.as_ref() {
			VisualBox::BlockBox(block) => block.add_child(child.clone()),
			VisualBox::InlineBox(inline) => inline.add_child(child.clone()),
			VisualBox::AnonymousBox(value) => value.add_child(child.clone()),
		}
		child.set_parent(Some(source));
	}

	fn get_last_child(&self) -> Option<Rc<VisualBox>> {
		match self {
			VisualBox::BlockBox(block) => block.get_last_child(),
			VisualBox::InlineBox(inline) => inline.get_last_child(),
			VisualBox::AnonymousBox(value) => value.get_last_child(),
		}
	}

	pub fn parent(&self) -> Option<Rc<VisualBox>> {
		match self {
			VisualBox::BlockBox(block) => block.parent(),
			VisualBox::InlineBox(inline) => inline.parent(),
			VisualBox::AnonymousBox(value) => value.parent(),
		}
	}

	pub fn set_parent(&self, value: Option<Rc<VisualBox>>) {
		match self {
			VisualBox::BlockBox(block) => block.set_parent(value),
			VisualBox::InlineBox(inline) => inline.set_parent(value),
			VisualBox::AnonymousBox(anonymous) => anonymous.set_parent(value),
		}
	}

	pub fn is_block_level(&self) -> bool {
		match self {
			VisualBox::BlockBox(_) => true,
			_ => false,
		}
	}

	pub fn is_inline_level(&self) -> bool {
		match self {
			VisualBox::InlineBox(_) => true,
			_ => false,
		}
	}

	pub fn is_anonymous(&self) -> bool {
		match self {
			VisualBox::AnonymousBox(_) => true,
			_ => false,
		}
	}
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum FormattingContextType {
	BlockFormattingContext,
	InlineFormattingContext,
}

pub struct FormattingContext {
	pub formatting_context_type: FormattingContextType,
	pub established_by: RefCell<Option<Weak<VisualBox>>>,
}

impl FormattingContext {
	pub fn new(context_type: FormattingContextType) -> Self {
		FormattingContext {
			established_by: RefCell::new(None),
			formatting_context_type: context_type,
		}
	}

	pub fn set_established_by(&self, owner: Rc<VisualBox>) {
		self.established_by.replace(Some(Rc::downgrade(&owner)));
	}
}

pub struct BoxTree {
	pub root: Rc<VisualBox>,
}

impl BoxTree {
	pub fn construct(style_tree: Rc<StyleTree>) -> BoxTree {
		let style_node = style_tree.root();
		let root = VisualBox::new_with_formatting_context(
			FormattingContextType::BlockFormattingContext,
			|formatting_context| {
				Rc::new(VisualBox::BlockBox(BlockLevelBox::new(
					style_node.dom_node.clone(),
					formatting_context,
				)))
			},
		);
		let children_iter = BoxTree::get_children_iter(style_node);
		for child in children_iter {
			let child_box = BoxTree::construct_node(child, root.clone());
			VisualBox::append_child(root.clone(), child_box);
		}
		BoxTree { root }
	}

	/// https://drafts.csswg.org/css-display/#outer-role
	/*
	- we currently only support [normal flow](https://www.w3.org/TR/CSS22/visuren.html#normal-flow) and non-replaced elements
	- if outer is inline -> generate inline level box
	  if inner is
		- flow -> keep the context as its parent
		- flow-root -> establish a new block formatting context
	  if parent has block formatting context, wrap inside annonymous block level box
	  (include its siblings if inline box) with inline formatting context
	- if outer is block -> generate block level box
	  if inner is
		- flow -> generate block-container box (1)
		- flow-root -> establish a new block formatting context
	  if parent has block formatting context -> nothing happens.
	  | Otherwise, traversing its ancestors until reaching block formatting context A (parent) and inline formatting context B (child),
		split B into B1-current-B2 (both B1 and B2 are wrapped in annonymous block level boxes) (https://www.w3.org/TR/CSS22/visuren.html#anonymous-block-level)
		and mark B1, B2 as splitted box
	- (1) depend on its children
		- if there is mixed between block and inline level boxes
		  reuse its parent context and wrap its children inline level box into annonymous block level box
		| else establish a inline formatting context
	*/
	fn construct_node(style_node: Rc<StyleTreeNode>, parent_box: Rc<VisualBox>) -> Rc<VisualBox> {
		let (outside, inside) = BoxTree::get_display(&style_node.dom_node);
		let visual_box = match outside {
			DisplayOutside::Inline => match inside {
				DisplayInside::Flow => Rc::new(VisualBox::InlineBox(InlineLevelBox::new(
					style_node.dom_node.clone(),
					parent_box.formatting_context(),
				))),
				DisplayInside::FlowRoot => VisualBox::new_with_formatting_context(
					FormattingContextType::BlockFormattingContext,
					|formatting_context| {
						Rc::new(VisualBox::InlineBox(InlineLevelBox::new(
							style_node.dom_node.clone(),
							formatting_context,
						)))
					},
				),
				_ => not_supported!(),
			},
			DisplayOutside::Block => match inside {
				DisplayInside::Flow => {
					if !BoxTree::is_contain_all_inline_children(style_node.clone()) {
						Rc::new(VisualBox::BlockBox(BlockLevelBox::new(
							style_node.dom_node.clone(),
							parent_box.formatting_context(),
						)))
					} else {
						VisualBox::new_with_formatting_context(
							FormattingContextType::InlineFormattingContext,
							|formatting_context| {
								Rc::new(VisualBox::BlockBox(BlockLevelBox::new(
									style_node.dom_node.clone(),
									formatting_context,
								)))
							},
						)
					}
				},
				DisplayInside::FlowRoot => VisualBox::new_with_formatting_context(
					FormattingContextType::BlockFormattingContext,
					|formatting_context| {
						Rc::new(VisualBox::BlockBox(BlockLevelBox::new(
							style_node.dom_node.clone(),
							formatting_context,
						)))
					},
				),
				_ => not_supported!(),
			},
			_ => not_supported!(),
		};
		VisualBox::append_child(parent_box, visual_box.clone());

		let children_iter = BoxTree::get_children_iter(style_node);
		for child in children_iter {
			let child_box = BoxTree::construct_node(child, visual_box.clone());
			VisualBox::append_child(visual_box.clone(), child_box);
		}
		visual_box
	}

	fn get_children_iter(style_node: Rc<StyleTreeNode>) -> impl Iterator<Item = Rc<StyleTreeNode>> {
		SimpleNodeIterator::new(style_node.first_child(), |n: &Rc<StyleTreeNode>| {
			let mut next_sibling = n.next_sibling();
			loop {
				if let Some(ref style_node) = next_sibling {
					let computed_values =
						GlobalScope::get_or_init_computed_values(style_node.dom_node.id());
					match computed_values.get_display() {
						longhands::display::Display::Box(value)
							if *value == longhands::display::DisplayBox::None =>
						{
							next_sibling = style_node.next_sibling();
							continue;
						}
						_ => (),
					}
				}
				break;
			}
			next_sibling
		})
	}

	fn is_contain_all_inline_children(style_node: Rc<StyleTreeNode>) -> bool {
		let children_iter = BoxTree::get_children_iter(style_node);
		for child in children_iter {
			let (outside, _) = BoxTree::get_display(&child.dom_node);
			if outside != DisplayOutside::Inline {
				return false;
			}
		}
		true
	}

	fn get_display(dom_node: &NodeRef) -> (DisplayOutside, DisplayInside) {
		let computed_values = GlobalScope::get_or_init_computed_values(dom_node.id());
		match computed_values.get_display() {
			longhands::display::Display::Basic(DisplayBasic { outside, inside }) => (
				outside
					.as_ref()
					.map_or(DisplayOutside::Block, |v| v.clone()),
				inside
					.as_ref()
					.clone()
					.map_or(DisplayInside::Flow, |v| v.clone()),
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
	}
}
