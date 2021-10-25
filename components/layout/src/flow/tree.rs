use std::collections::VecDeque;
use std::rc::Rc;

use common::{not_reached, not_supported};
use css::properties::longhands;
use css::properties::longhands::display::{DisplayBasic, DisplayInside, DisplayOutside};
use css::values::{Pixel, PIXEL_ZERO};
use dom::characterdata::CharacterData;
use dom::global_scope::{GlobalScope, NodeRef};
use dom::inheritance::Castable;
use dom::node::{Node, SimpleNodeIterator};
use dom::nodetype::NodeTypeId;

use super::boxes::{BlockLevelBox, InlineLevelBox, VisualBox};
use super::formatting_context::FormattingContextType;
use crate::style_tree::{StyleTree, StyleTreeNode};
use crate::text::TextUI;

pub struct BoxTree {
	pub root: Rc<VisualBox>,
	pub initial_containing_block: Rc<VisualBox>,
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
		let initial_containing_block = VisualBox::new_with_formatting_context(
			FormattingContextType::BlockFormattingContext,
			|formatting_context| {
				Rc::new(VisualBox::BlockBox(BlockLevelBox::new(
					NodeRef(Rc::new(Node::new(NodeTypeId::Document, None))),
					formatting_context,
				)))
			},
		);
		initial_containing_block.size().width = Pixel::new(
			style_node
				.dom_node
				.window()
				.expect("dom has to belong to a window")
				.viewport()
				.width(),
		);
		root.set_containing_block(Some(initial_containing_block.clone()));
		let children_iter = BoxTree::get_children_iter(style_node);
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
	fn construct_node(style_node: Rc<StyleTreeNode>, parent_box: Rc<VisualBox>) {
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
		BoxTree::set_containing_box(visual_box.clone());

		let children_iter = BoxTree::get_children_iter(style_node);
		for child in children_iter {
			BoxTree::construct_node(child, visual_box.clone());
		}
	}

	pub fn set_containing_box(src: Rc<VisualBox>) {
		let mut containing_block = None;
		for ancestor in src.ancestors() {
			// TODO: include box which establishes a new formatting context
			// https://www.w3.org/TR/CSS22/visudet.html#containing-block-details
			if ancestor.is_block_container() {
				containing_block = Some(ancestor);
				break;
			}
		}
		if let Some(containing_block) = containing_block {
			src.set_containing_block(Some(containing_block.clone()));
			// during constructing box tree, there might be anonymous boxes which are added (as its parent) to ensure https://www.w3.org/TR/CSS22/visuren.html#anonymous-block-level
			for ancestor in src.ancestors() {
				match ancestor.as_ref() {
					VisualBox::AnonymousBox(anonymous)
						if anonymous.containing_block().is_none() =>
					{
						anonymous.set_containing_block(Some(containing_block.clone()))
					},
					_ => break,
				}
			}
		} else {
			panic!("one of box's ancestors must be its containing box");
		}
	}

	pub fn log(&self) {
		self.log_node(self.root.clone(), 0);
	}

	fn log_node(&self, node: Rc<VisualBox>, depth: usize) {
		let indent: String = std::iter::repeat("  ").take(depth).collect();
		match node.as_ref() {
			VisualBox::BlockBox(block) => {
				println!(
					"{}block-{:?}-{:?}-{:?}",
					indent,
					block.dom_node().node_type_id(),
					block.formatting_context_type(),
					block.size().width,
				)
			},
			VisualBox::InlineBox(inline) => {
				println!(
					"{}inline-{:?}-{:?}-{:?}",
					indent,
					inline.dom_node().node_type_id(),
					inline.formatting_context_type(),
					inline.size().width
				)
			},
			VisualBox::AnonymousBox(anonymous) => println!(
				"{}anonymous-{:?}-{:?}",
				indent,
				anonymous.formatting_context_type(),
				anonymous.size().width
			),
		};
		for child in node.children() {
			self.log_node(child, depth + 1);
		}
	}

	pub fn get_total_children_intrinsic_width(src: Rc<VisualBox>) -> Pixel {
		let mut total_children_width = PIXEL_ZERO;
		for child in src.children() {
			match src.formatting_context_type() {
				FormattingContextType::BlockFormattingContext => {
					total_children_width = child.size().intrinsic_width.max(total_children_width);
				},
				FormattingContextType::InlineFormattingContext => {
					total_children_width += child.size().intrinsic_width;
				},
			}
		}
		total_children_width
	}

	/*
	we perform multiple traversals to incremental figure out the used value for every elements:
		- post-order traversal to compute the intrinsic width for elements.
		- pre-order traversal to compute used value for width for elements.
		- post-order traversal to compute height for block elements.
	 */
	pub fn compute_layout(&self) {
		self.bubble_intrinsic_inline_size();
		self.compute_used_value();
		// self.bubble_height();
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
	pub fn bubble_intrinsic_inline_size(&self) {
		let node_iter = PostOrderBoxTreeIterator::new(self.root.clone());
		for node in node_iter {
			let width = match node.as_ref() {
				VisualBox::InlineBox(inline)
					if inline.formatting_context_type()
						== FormattingContextType::InlineFormattingContext =>
				{
					if inline.children().len() > 0 {
						let mut total_children_width = PIXEL_ZERO;
						for child in inline.children() {
							total_children_width += child.size().intrinsic_width;
						}
						total_children_width
					} else if inline.dom_node().node_type_id().is_character_data_text() {
						let dom_node = inline.dom_node();
						let content = dom_node.0.downcast::<CharacterData>().data();
						let computed_values = GlobalScope::get_or_init_computed_values(
							dom_node
								.parent_node()
								.expect("dom has to have a parent")
								.id(),
						);
						let family_names = computed_values.get_font_families();
						Pixel::new(
							TextUI::new()
								.measure_size(content.as_str(), family_names, 14.0)
								.0,
						)
					} else {
						PIXEL_ZERO
					}
				},
				_ => BoxTree::get_total_children_intrinsic_width(node.clone()),
			};
			node.size().intrinsic_width = width;
		}
	}

	/*
	if box is block-level box
		- its width + layout box (padding, margin) = width of its containing block (https://www.w3.org/TR/CSS22/visudet.html#blockwidth),
		- if element has no parent (it is root), it belongs to the initial containing block which is a viewport.
	if box is inline-level box and its formatting context:
		- block formatting context:
			- if width = auto, its width = min(total children intrinsic width, containing block width) (https://www.w3.org/TR/CSS22/visudet.html#shrink-to-fit-float.)
			- if width in px, its width = its intrinsic width.
			- if width in percentage, its width = its containing block width * percentage.
		- inline formatting context, its width = its intrinsic width
	skip anonymous box (since it doesn't have width, only intrinsic width which is calculated in the previous step)
	*/
	pub fn compute_used_value(&self) {
		let node_iter = PreOrderBoxTreeIterator::new(self.root.clone());
		for node in node_iter {
			let containing_block = node
				.containing_block()
				.expect("box has to have a containing block");
			match node.as_ref() {
				VisualBox::BlockBox(block) => block.compute_used_value(containing_block),
				VisualBox::InlineBox(inline) => {
					inline.compute_used_value(containing_block, node.clone())
				},
				VisualBox::AnonymousBox(anonymous) => {
					anonymous.size().set_width(containing_block.size().width)
				},
			}
		}
	}

	pub fn bubble_height(&self) {
		todo!()
	}

	fn get_children_iter(style_node: Rc<StyleTreeNode>) -> impl Iterator<Item = Rc<StyleTreeNode>> {
		fn adjust_current_node(node: Option<Rc<StyleTreeNode>>) -> Option<Rc<StyleTreeNode>> {
			let mut current = node;
			while let Some(ref style_node) = current {
				let computed_values =
					GlobalScope::get_or_init_computed_values(style_node.dom_node.id());
				match computed_values.get_display() {
					longhands::display::Display::Box(value)
						if *value == longhands::display::DisplayBox::None =>
					{
						current = style_node.next_sibling();
						continue;
					}
					_ => break,
				}
			}
			current
		}
		SimpleNodeIterator::new(
			adjust_current_node(style_node.first_child()),
			|n: &Rc<StyleTreeNode>| adjust_current_node(n.next_sibling()),
		)
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
		if dom_node.node_type_id().is_element() {
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
		} else {
			(DisplayOutside::Inline, DisplayInside::Flow)
		}
	}
}

#[derive(PartialEq, Eq)]
enum TraversalState {
	Up,
	Down,
}

pub struct PostOrderBoxTreeIterator {
	current: Option<(usize, Rc<VisualBox>)>,
	stack: VecDeque<(usize, Rc<VisualBox>)>,
	state: TraversalState,
}

impl PostOrderBoxTreeIterator {
	pub fn new(root: Rc<VisualBox>) -> PostOrderBoxTreeIterator {
		PostOrderBoxTreeIterator {
			current: Some((0, root)),
			stack: VecDeque::with_capacity(1),
			state: TraversalState::Down,
		}
	}

	fn build_left_branch(&mut self, index: usize, value: Rc<VisualBox>) {
		self.stack.push_back((index, value.clone()));
		let mut node = value;
		while let Some(first_child) = node.get_first_child() {
			node = first_child.clone();
			self.stack.push_back((0, first_child));
			self.state = TraversalState::Down;
		}
	}

	fn move_next(&mut self, index: usize, value: Rc<VisualBox>) {
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
	type Item = Rc<VisualBox>;

	fn next(&mut self) -> Option<Rc<VisualBox>> {
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
	current: Option<(usize, Rc<VisualBox>)>,
	stack: VecDeque<(usize, Rc<VisualBox>)>,
}

impl PreOrderBoxTreeIterator {
	pub fn new(root: Rc<VisualBox>) -> PreOrderBoxTreeIterator {
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
	type Item = Rc<VisualBox>;

	fn next(&mut self) -> Option<Rc<VisualBox>> {
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
