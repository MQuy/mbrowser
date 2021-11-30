use std::collections::VecDeque;
use std::rc::Rc;

use common::not_supported;
use css::properties::longhands::display::{DisplayInside, DisplayOutside};
use css::values::{Pixel, PIXEL_ZERO};
use dom::global_scope::NodeRef;
use dom::node::Node;
use dom::nodetype::NodeTypeId;

use super::block::BlockLevelBox;
use super::boxes::{Box, BoxClass};
use super::formatting_context::FormattingContextType;
use super::fragment::LayoutInfo;
use crate::flow::inline::InlineLevelBox;
use crate::flow::text_run::TextRun;
use crate::style_tree::{StyleTree, StyleTreeNode};

pub struct BoxTree {
	pub root: Rc<dyn Box>,
	pub initial_containing_block: Rc<dyn Box>,
}

impl BoxTree {
	pub fn construct(style_tree: Rc<StyleTree>) -> BoxTree {
		let style_node = style_tree.root();
		let root = BoxClass::new_with_formatting_context(
			FormattingContextType::BlockFormattingContext,
			|formatting_context| Rc::new(BlockLevelBox::new(style_node.dom_node.clone(), formatting_context)),
		);
		let viewport = style_node
			.dom_node
			.window()
			.expect("dom has to belong to a window")
			.viewport()
			.clone();
		let initial_containing_block = BoxClass::new_with_formatting_context(
			FormattingContextType::BlockFormattingContext,
			|formatting_context| {
				let block = BlockLevelBox::new(
					NodeRef(Rc::new(Node::new(NodeTypeId::Document, None))),
					formatting_context,
				);
				block.set_layout_info(LayoutInfo {
					width: Pixel::new(viewport.width()),
					height: Pixel::new(viewport.height()),
					..Default::default()
				});
				block.set_fragment(block.create_fragment());
				Rc::new(block)
			},
		);
		root.set_containing_block(Some(initial_containing_block.clone()));
		let children_iter = style_node.get_visible_children_iter();
		for child in children_iter {
			BoxTree::construct_node(child, root.clone());
		}

		BoxTree {
			root,
			initial_containing_block,
		}
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
	fn construct_node(style_node: Rc<StyleTreeNode>, parent_box: Rc<dyn Box>) {
		let (outside, inside) = style_node.get_display();
		let visual_box = match outside {
			DisplayOutside::Inline => match inside {
				DisplayInside::Flow => {
					let node: Rc<dyn Box> = if style_node.dom_node.node_type_id().is_element() {
						Rc::new(InlineLevelBox::new(
							style_node.dom_node.clone(),
							parent_box.formatting_context(),
						))
					} else {
						Rc::new(TextRun::new(
							style_node.dom_node.clone(),
							parent_box.formatting_context(),
						))
					};
					node
				},
				DisplayInside::FlowRoot => BoxClass::new_with_formatting_context(
					FormattingContextType::BlockFormattingContext,
					|formatting_context| Rc::new(InlineLevelBox::new(style_node.dom_node.clone(), formatting_context)),
				),
				_ => not_supported!(),
			},
			DisplayOutside::Block => match inside {
				DisplayInside::Flow => {
					if !style_node.is_contain_all_inline_children() {
						Rc::new(BlockLevelBox::new(
							style_node.dom_node.clone(),
							parent_box.formatting_context(),
						))
					} else {
						BoxClass::new_with_formatting_context(
							FormattingContextType::InlineFormattingContext,
							|formatting_context| {
								Rc::new(BlockLevelBox::new(style_node.dom_node.clone(), formatting_context))
							},
						)
					}
				},
				DisplayInside::FlowRoot => BoxClass::new_with_formatting_context(
					FormattingContextType::BlockFormattingContext,
					|formatting_context| Rc::new(BlockLevelBox::new(style_node.dom_node.clone(), formatting_context)),
				),
				_ => not_supported!(),
			},
			_ => not_supported!(),
		};
		BoxClass::append_child(parent_box, visual_box.clone());
		BoxClass::set_containing_box(visual_box.clone());

		let children_iter = style_node.get_visible_children_iter();
		for child in children_iter {
			BoxTree::construct_node(child, visual_box.clone());
		}
	}

	pub fn log(&self) {
		self.log_node(self.root.clone(), 0);
	}

	fn log_node(&self, node: Rc<dyn Box>, depth: usize) {
		let indent: String = std::iter::repeat("  ").take(depth).collect();
		match node.class() {
			BoxClass::Block => {
				let block = node.as_block_level_box();
				println!(
					"{}block-{:?}-{:?}",
					indent,
					block.dom_node().node_type_id(),
					node.formatting_context_type(),
				)
			},
			BoxClass::Inline => {
				let inline = node.as_inline_level_box();
				println!(
					"{}inline-{:?}-{:?}",
					indent,
					inline.dom_node().node_type_id(),
					node.formatting_context_type(),
				)
			},
			BoxClass::Anonymous => {
				println!("{}anonymous-{:?}", indent, node.formatting_context_type())
			},
			BoxClass::TextRun => {
				println!("{}text run-{:?}", indent, node.formatting_context_type())
			},
		};
		for child in node.children() {
			self.log_node(child, depth + 1);
		}
	}

	/*
	we perform multiple traversals to incremental figure out the used value for every elements:
		- post-order traversal to compute the intrinsic width for elements.
		- pre-order traversal to compute used value for width for elements.
		- post-order traversal to compute height for block elements.
	 */
	pub fn compute_layout(&self) {
		self.prepare_layout();
		self.visit_layout();
	}

	/*
	if box is inline-level box and its formatting context is inline formatting context (ignore css width):
		- if box has children, its intrinsic width = sum of all children's width.
		- If box has no children and is text node, width = text's width.
		- otherwise, box's width = 0.
	otherwise
		- if box is inline-level box (block formatting context) -> its intrinsic width = total children's width
		- if box is block-level box -> its intrinsic width = maximum from each child's width
	*/
	pub fn prepare_layout(&self) {
		let node_iter = PostOrderBoxTreeIterator::new(self.root.clone());
		for node in node_iter {
			node.prepare_layout();
		}
	}

	pub fn visit_layout(&self) {
		let mut context = VisitingContext { height: PIXEL_ZERO };
		self.visit_layout_node(self.root.clone(), &mut context);
	}

	pub fn visit_layout_node(&self, node: Rc<dyn Box>, parent_context: &mut VisitingContext) {
		node.visit_layout();
		let mut context = VisitingContext { height: PIXEL_ZERO };
		for child in node.children() {
			self.visit_layout_node(child, &mut context);
		}
		node.revisit_layout(parent_context);
	}
}

pub struct VisitingContext {
	pub height: Pixel,
}

#[derive(PartialEq, Eq)]
enum TraversalState {
	Up,
	Down,
}

pub struct PostOrderBoxTreeIterator {
	current: Option<(usize, Rc<dyn Box>)>,
	stack: VecDeque<(usize, Rc<dyn Box>)>,
	state: TraversalState,
}

impl PostOrderBoxTreeIterator {
	pub fn new(root: Rc<dyn Box>) -> PostOrderBoxTreeIterator {
		PostOrderBoxTreeIterator {
			current: Some((0, root)),
			stack: VecDeque::with_capacity(1),
			state: TraversalState::Down,
		}
	}

	fn build_left_branch(&mut self, index: usize, value: Rc<dyn Box>) {
		self.stack.push_back((index, value.clone()));
		let mut node = value;
		while let Some(first_child) = node.get_first_child() {
			node = first_child.clone();
			self.stack.push_back((0, first_child));
			self.state = TraversalState::Down;
		}
	}

	fn move_next(&mut self, index: usize, value: Rc<dyn Box>) {
		if value.parent().is_none() || self.stack.len() == 0 {
			self.current = None;
			return;
		} else if let Some(parent) = value.parent() {
			if let Some(sibling) = parent.children().get(index + 1) {
				self.state = TraversalState::Down;
				self.current = Some((index + 1, sibling.clone()))
			} else if let Some((index, value)) = self.stack.front() {
				self.state = TraversalState::Up;
				self.current = Some((index.clone(), value.clone()))
			} else {
				unreachable!()
			}
		}
	}
}

impl Iterator for PostOrderBoxTreeIterator {
	type Item = Rc<dyn Box>;

	fn next(&mut self) -> Option<Rc<dyn Box>> {
		let (index, current) = self.current.take()?;
		if self.state == TraversalState::Down {
			self.build_left_branch(index, current);
		};
		let (index, value) = self.stack.pop_back()?;
		self.move_next(index, value.clone());
		Some(value)
	}
}

pub struct PreOrderBoxTreeIterator {
	current: Option<(usize, Rc<dyn Box>)>,
	stack: VecDeque<(usize, Rc<dyn Box>)>,
}

impl PreOrderBoxTreeIterator {
	pub fn new(root: Rc<dyn Box>) -> PreOrderBoxTreeIterator {
		PreOrderBoxTreeIterator {
			current: Some((0, root)),
			stack: VecDeque::with_capacity(1),
		}
	}

	pub fn move_next(&mut self) {
		while let Some((index, _)) = self.stack.pop_front() {
			if let Some((_, parent)) = self.stack.front() {
				if let Some(next_sibling) = parent.children().get(index + 1) {
					self.current = Some((index + 1, next_sibling.clone()));
					return;
				} else {
					continue;
				}
			}
		}
		self.current = None;
	}
}

impl Iterator for PreOrderBoxTreeIterator {
	type Item = Rc<dyn Box>;

	fn next(&mut self) -> Option<Rc<dyn Box>> {
		let (index, current) = self.current.take()?;
		self.stack.push_back((index, current.clone()));
		if let Some(first_child) = current.get_first_child() {
			self.current = Some((0, first_child));
		} else {
			self.move_next();
		}
		Some(current)
	}
}
