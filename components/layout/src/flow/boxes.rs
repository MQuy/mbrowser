use std::cell::{RefCell, RefMut};
use std::fmt::format;
use std::rc::{Rc, Weak};

use common::{not_reached, not_supported};
use css::values::computed::length::{LengthPercentage, LengthPercentageOrAuto, Size};
use css::values::{Pixel, PIXEL_ZERO};
use dom::global_scope::{GlobalScope, NodeRef};
use dom::node::SimpleNodeIterator;

use super::dimension::BoxDimension;
use super::formatting_context::{FormattingContext, FormattingContextType};
use crate::flow::tree::BoxTree;

pub struct BaseBox {
	pub formatting_context: RefCell<Rc<FormattingContext>>,
	pub children: RefCell<Vec<Rc<VisualBox>>>,
	pub parent: RefCell<Option<Weak<VisualBox>>>,
	pub containing_block: RefCell<Option<Weak<VisualBox>>>,
	pub size: RefCell<BoxDimension>,
}

impl BaseBox {
	pub fn new(formatting_context: Rc<FormattingContext>) -> Self {
		BaseBox {
			parent: RefCell::new(Default::default()),
			formatting_context: RefCell::new(formatting_context),
			children: RefCell::new(Default::default()),
			containing_block: RefCell::new(Default::default()),
			size: RefCell::new(Default::default()),
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

	pub fn get_first_child(&self) -> Option<Rc<VisualBox>> {
		self.children.borrow().first().map(|value| value.clone())
	}

	pub fn get_last_child(&self) -> Option<Rc<VisualBox>> {
		self.children.borrow().last().map(|value| value.clone())
	}

	pub fn children(&self) -> Vec<Rc<VisualBox>> {
		self.children.borrow().clone()
	}

	pub fn parent(&self) -> Option<Rc<VisualBox>> {
		match self.parent.borrow().as_ref() {
			Some(value) => value.upgrade(),
			None => None,
		}
	}

	pub fn containing_block(&self) -> Option<Rc<VisualBox>> {
		match self.containing_block.borrow().as_ref() {
			Some(value) => value.upgrade(),
			None => None,
		}
	}

	pub fn set_containing_block(&self, value: Option<Rc<VisualBox>>) {
		self.containing_block
			.replace(value.as_ref().map(|v| Rc::downgrade(v)));
	}

	pub fn set_parent(&self, value: Option<Rc<VisualBox>>) {
		self.parent
			.replace(value.as_ref().map(|v| Rc::downgrade(v)));
	}

	pub fn size(&self) -> RefMut<'_, BoxDimension> {
		self.size.borrow_mut()
	}
}

/// https://www.w3.org/TR/CSS22/visuren.html#block-boxes
pub struct BlockLevelBox {
	dom_node: NodeRef,
	base: BaseBox,
}

impl BlockLevelBox {
	pub fn new(dom_node: NodeRef, formatting_context: Rc<FormattingContext>) -> Self {
		BlockLevelBox {
			base: BaseBox::new(formatting_context),
			dom_node,
		}
	}

	pub fn add_child(&self, child: Rc<VisualBox>) {
		self.base.add_child(child)
	}

	pub fn dom_node(&self) -> NodeRef {
		self.dom_node.clone()
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

	pub fn get_first_child(&self) -> Option<Rc<VisualBox>> {
		self.base.get_first_child()
	}

	pub fn get_last_child(&self) -> Option<Rc<VisualBox>> {
		self.base.get_last_child()
	}

	pub fn children(&self) -> Vec<Rc<VisualBox>> {
		self.base.children()
	}

	pub fn parent(&self) -> Option<Rc<VisualBox>> {
		self.base.parent()
	}

	pub fn set_parent(&self, value: Option<Rc<VisualBox>>) {
		self.base.set_parent(value);
	}

	pub fn containing_block(&self) -> Option<Rc<VisualBox>> {
		self.base.containing_block()
	}

	pub fn set_containing_block(&self, value: Option<Rc<VisualBox>>) {
		self.base.set_containing_block(value);
	}

	pub fn size(&self) -> RefMut<'_, BoxDimension> {
		self.base.size()
	}

	pub fn compute_used_value(&self, containing_block: Rc<VisualBox>) {
		let computed_values = GlobalScope::get_or_init_computed_values(self.dom_node().id());
		let containing_width = containing_block.size().width;
		let padding_left = computed_values
			.get_padding_left()
			.to_used_value(containing_width);
		let padding_right = computed_values
			.get_padding_right()
			.to_used_value(containing_width);
		let mut margin_left = PIXEL_ZERO;
		let mut margin_right = PIXEL_ZERO;
		let width = match computed_values.get_width() {
			Size::Auto => {
				margin_left = computed_values
					.get_margin_left()
					.to_used_value(containing_width, PIXEL_ZERO);
				margin_right = computed_values
					.get_margin_left()
					.to_used_value(containing_width, PIXEL_ZERO);
				PIXEL_ZERO.max(
					containing_width - margin_left - padding_left - padding_right - margin_right,
				)
			},
			Size::LengthPercentage(length_percentage) => {
				let width = length_percentage.to_used_value(containing_width);
				let margin = containing_width - width - padding_left - padding_right;
				if margin < PIXEL_ZERO {
					if *computed_values.get_margin_left() == LengthPercentageOrAuto::Auto {
						margin_left = PIXEL_ZERO;
					}
					if *computed_values.get_margin_right() == LengthPercentageOrAuto::Auto {
						margin_right = PIXEL_ZERO;
					}
				} else {
					if *computed_values.get_margin_left() == LengthPercentageOrAuto::Auto
						&& *computed_values.get_margin_right() == LengthPercentageOrAuto::Auto
					{
						margin_left = margin / 2.0;
						margin_right = margin / 2.0;
					} else if *computed_values.get_margin_left() == LengthPercentageOrAuto::Auto {
						margin_right = computed_values
							.get_margin_right()
							.to_used_value(containing_width, PIXEL_ZERO);
						margin_left = margin - margin_right;
					} else if *computed_values.get_margin_right() == LengthPercentageOrAuto::Auto {
						margin_left = computed_values
							.get_margin_left()
							.to_used_value(containing_width, PIXEL_ZERO);
						margin_right = margin - margin_left;
					}
				};
				width
			},
			css::values::generics::length::GenericSize::ExtremumLength(_) => {
				not_supported!()
			},
		};
		let mut dimentions = self.size();
		dimentions.set_padding_left(padding_left);
		dimentions.set_padding_right(padding_right);
		dimentions.set_margin_left(margin_left);
		dimentions.set_margin_right(margin_right);
		dimentions.set_width(width);
	}
}

/// https://www.w3.org/TR/CSS22/visuren.html#inline-boxes
pub struct InlineLevelBox {
	dom_node: NodeRef,
	base: BaseBox,
}

impl InlineLevelBox {
	pub fn new(dom_node: NodeRef, formatting_context: Rc<FormattingContext>) -> Self {
		InlineLevelBox {
			base: BaseBox::new(formatting_context),
			dom_node,
		}
	}

	pub fn dom_node(&self) -> NodeRef {
		self.dom_node.clone()
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

	pub fn get_first_child(&self) -> Option<Rc<VisualBox>> {
		self.base.get_first_child()
	}

	pub fn get_last_child(&self) -> Option<Rc<VisualBox>> {
		self.base.get_last_child()
	}

	pub fn children(&self) -> Vec<Rc<VisualBox>> {
		self.base.children()
	}

	pub fn parent(&self) -> Option<Rc<VisualBox>> {
		self.base.parent()
	}

	pub fn set_parent(&self, value: Option<Rc<VisualBox>>) {
		self.base.set_parent(value);
	}

	pub fn containing_block(&self) -> Option<Rc<VisualBox>> {
		self.base.containing_block()
	}

	pub fn set_containing_block(&self, value: Option<Rc<VisualBox>>) {
		self.base.set_containing_block(value);
	}

	pub fn size(&self) -> RefMut<'_, BoxDimension> {
		self.base.size()
	}

	pub fn compute_used_value(&self, containing_block: Rc<VisualBox>, src: Rc<VisualBox>) {
		let computed_values = GlobalScope::get_or_init_computed_values(self.dom_node().id());
		let containing_width = containing_block.size().width;
		let padding_left = computed_values
			.get_padding_left()
			.to_used_value(containing_width);
		let padding_right = computed_values
			.get_padding_right()
			.to_used_value(containing_width);
		let margin_left = computed_values
			.get_margin_left()
			.to_used_value(containing_width, PIXEL_ZERO);
		let margin_right = computed_values
			.get_margin_right()
			.to_used_value(containing_width, PIXEL_ZERO);
		let width = match self.formatting_context_type() {
			FormattingContextType::BlockFormattingContext => match computed_values.get_width() {
				Size::Auto => BoxTree::get_total_children_intrinsic_width(src).min(
					containing_width - margin_left - padding_left - padding_right - margin_right,
				),
				Size::LengthPercentage(value) => match value.0.clone() {
					LengthPercentage::AbsoluteLength(length) => Pixel::new(length),
					LengthPercentage::Percentage(percentage) => {
						containing_width * percentage.to_value(&(0.0..1.0))
					},
				},
				_ => not_supported!(),
			},
			FormattingContextType::InlineFormattingContext => {
				self.size().intrinsic_width.min(containing_width)
			},
		};
		let mut dimentions = self.size();
		dimentions.set_padding_left(padding_left);
		dimentions.set_padding_right(padding_right);
		dimentions.set_margin_left(margin_left);
		dimentions.set_margin_right(margin_right);
		dimentions.set_width(width);
	}
}

// Anonymous box is always anonymous block box
pub struct AnonymousBox {
	base: BaseBox,
}

impl AnonymousBox {
	pub fn new(formatting_context: Rc<FormattingContext>) -> Self {
		AnonymousBox {
			base: BaseBox::new(formatting_context),
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

	pub fn get_first_child(&self) -> Option<Rc<VisualBox>> {
		self.base.get_first_child()
	}

	pub fn get_last_child(&self) -> Option<Rc<VisualBox>> {
		self.base.get_last_child()
	}

	pub fn children(&self) -> Vec<Rc<VisualBox>> {
		self.base.children()
	}

	pub fn parent(&self) -> Option<Rc<VisualBox>> {
		self.base.parent()
	}

	pub fn set_parent(&self, value: Option<Rc<VisualBox>>) {
		self.base.set_parent(value);
	}

	pub fn containing_block(&self) -> Option<Rc<VisualBox>> {
		self.base.containing_block()
	}

	pub fn set_containing_block(&self, value: Option<Rc<VisualBox>>) {
		self.base.set_containing_block(value)
	}

	pub fn size(&self) -> RefMut<'_, BoxDimension> {
		self.base.size()
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
				child.set_formatting_context(anonymous_box.formatting_context());
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

	pub fn get_first_child(&self) -> Option<Rc<VisualBox>> {
		match self {
			VisualBox::BlockBox(block) => block.get_first_child(),
			VisualBox::InlineBox(inline) => inline.get_first_child(),
			VisualBox::AnonymousBox(anonymous) => anonymous.get_first_child(),
		}
	}

	pub fn get_last_child(&self) -> Option<Rc<VisualBox>> {
		match self {
			VisualBox::BlockBox(block) => block.get_last_child(),
			VisualBox::InlineBox(inline) => inline.get_last_child(),
			VisualBox::AnonymousBox(value) => value.get_last_child(),
		}
	}

	pub fn children(&self) -> Vec<Rc<VisualBox>> {
		match self {
			VisualBox::BlockBox(block) => block.children(),
			VisualBox::InlineBox(inline) => inline.children(),
			VisualBox::AnonymousBox(anonymous) => anonymous.children(),
		}
	}

	pub fn parent(&self) -> Option<Rc<VisualBox>> {
		match self {
			VisualBox::BlockBox(block) => block.parent(),
			VisualBox::InlineBox(inline) => inline.parent(),
			VisualBox::AnonymousBox(value) => value.parent(),
		}
	}

	pub fn ancestors(&self) -> impl Iterator<Item = Rc<VisualBox>> {
		SimpleNodeIterator::new(self.parent(), |n: &Rc<VisualBox>| n.parent())
	}

	pub fn set_parent(&self, value: Option<Rc<VisualBox>>) {
		match self {
			VisualBox::BlockBox(block) => block.set_parent(value),
			VisualBox::InlineBox(inline) => inline.set_parent(value),
			VisualBox::AnonymousBox(anonymous) => anonymous.set_parent(value),
		}
	}

	pub fn containing_block(&self) -> Option<Rc<VisualBox>> {
		match self {
			VisualBox::BlockBox(block) => block.containing_block(),
			VisualBox::InlineBox(inline) => inline.containing_block(),
			VisualBox::AnonymousBox(anonymous) => anonymous.containing_block(),
		}
	}

	pub fn set_containing_block(&self, value: Option<Rc<VisualBox>>) {
		match self {
			VisualBox::BlockBox(block) => block.set_containing_block(value),
			VisualBox::InlineBox(inline) => inline.set_containing_block(value),
			VisualBox::AnonymousBox(_) => not_reached!(),
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

	pub fn is_inline_block(&self) -> bool {
		match self {
			VisualBox::InlineBox(inline) => {
				inline.formatting_context_type() == FormattingContextType::BlockFormattingContext
			},
			_ => false,
		}
	}

	pub fn size(&self) -> RefMut<'_, BoxDimension> {
		match self {
			VisualBox::BlockBox(block) => block.size(),
			VisualBox::InlineBox(inline) => inline.size(),
			VisualBox::AnonymousBox(anonymous) => anonymous.size(),
		}
	}

	pub fn is_block_container(&self) -> bool {
		match self {
			VisualBox::BlockBox(_) => true,
			VisualBox::InlineBox(inline)
				if inline.formatting_context_type()
					== FormattingContextType::BlockFormattingContext =>
			{
				true
			},
			_ => false,
		}
	}
}
