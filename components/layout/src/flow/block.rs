use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

use common::not_supported;
use css::values::computed::length::{LengthPercentage, LengthPercentageOrAuto, Size};
use css::values::{Pixel, PIXEL_ZERO};
use dom::global_scope::{GlobalScope, NodeRef};

use super::boxes::{BaseBox, Box, BoxClass, SimpleBoxIterator};
use super::formatting_context::{FormattingContext, FormattingContextType};
use super::fragment::{BoxFragment, Fragment, LayoutInfo, Line};
use crate::display_list::builder::DisplayListBuilder;

/// https://www.w3.org/TR/CSS22/visuren.html#block-boxes
pub struct BlockLevelBox {
	dom_node: NodeRef,
	base: BaseBox,
	fragment: Rc<RefCell<BoxFragment>>,
	lines: RefCell<Vec<Line>>,
}

impl BlockLevelBox {
	pub fn new(dom_node: NodeRef, formatting_context: Rc<FormattingContext>) -> Self {
		BlockLevelBox {
			dom_node: dom_node.clone(),
			base: BaseBox::new(formatting_context),
			fragment: Rc::new(RefCell::new(BoxFragment::new(dom_node))),
			lines: RefCell::new(vec![Line::new()]),
		}
	}

	pub fn dom_node(&self) -> NodeRef {
		self.dom_node.clone()
	}

	pub fn fragment(&self) -> Ref<'_, BoxFragment> {
		self.fragment.borrow()
	}

	pub fn fragment_mut(&self) -> RefMut<'_, BoxFragment> {
		self.fragment.borrow_mut()
	}

	pub fn set_layout_info(&self, value: LayoutInfo) {
		self.base.layout_info.replace(value);
	}

	pub fn set_fragment(&self, value: BoxFragment) {
		self.fragment.replace(value);
	}

	pub fn create_fragment(&self) -> BoxFragment {
		let layout_info = self.layout_info();
		let mut fragment = BoxFragment::new(self.dom_node.clone());
		fragment.padding = layout_info.padding;
		fragment.margin = layout_info.margin;
		fragment.set_width(layout_info.width);
		fragment.set_bounded_width(layout_info.width);
		fragment.set_height(layout_info.height);
		fragment.set_bounded_height(layout_info.height);
		fragment
	}
}

impl Box for BlockLevelBox {
	fn id(&self) -> uuid::Uuid {
		self.base.id
	}

	fn add_child(&self, child: Rc<dyn Box>) {
		self.base.add_child(child)
	}

	fn formatting_context(&self) -> Rc<FormattingContext> {
		self.base.formatting_context()
	}

	fn set_formatting_context(&self, formatting_context: Rc<FormattingContext>) {
		self.base.set_formatting_context(formatting_context);
	}

	fn formatting_context_type(&self) -> FormattingContextType {
		self.base.formatting_context_type()
	}

	fn get_first_child(&self) -> Option<Rc<dyn Box>> {
		self.base.get_first_child()
	}

	fn get_last_child(&self) -> Option<Rc<dyn Box>> {
		self.base.get_last_child()
	}

	fn children(&self) -> Vec<Rc<dyn Box>> {
		self.base.children()
	}

	fn parent(&self) -> Option<Rc<dyn Box>> {
		self.base.parent()
	}

	fn set_parent(&self, value: Option<Rc<dyn Box>>) {
		self.base.set_parent(value);
	}

	fn ancestors(&self) -> SimpleBoxIterator {
		SimpleBoxIterator::new(self.parent(), &|n: &Rc<dyn Box>| n.parent())
	}

	fn containing_block(&self) -> Option<Rc<dyn Box>> {
		self.base.containing_block()
	}

	fn set_containing_block(&self, value: Option<Rc<dyn Box>>) {
		self.base.set_containing_block(value);
	}

	fn lines(&self) -> Ref<Vec<Line>> {
		self.lines.borrow()
	}

	fn lines_mut(&self) -> RefMut<Vec<Line>> {
		self.lines.borrow_mut()
	}

	fn layout_info(&self) -> Ref<'_, LayoutInfo> {
		self.base.layout_info.borrow()
	}

	fn layout_info_mut(&self) -> RefMut<'_, LayoutInfo> {
		self.base.layout_info.borrow_mut()
	}

	fn add_child_fragment(&self, fragment: Rc<RefCell<dyn Fragment>>) {
		self.fragment.borrow_mut().children.push(fragment);
	}

	fn get_total_width(&self) -> Pixel {
		let fragment = self.fragment();
		fragment.margin.left
			+ fragment.padding.left
			+ fragment.rect.width()
			+ fragment.padding.right
			+ fragment.margin.right
	}

	fn get_total_height(&self) -> Pixel {
		let fragment = self.fragment();
		fragment.margin.top
			+ fragment.padding.top
			+ fragment.rect.height()
			+ fragment.padding.bottom
			+ fragment.margin.bottom
	}

	fn is_block_container(&self) -> bool {
		true
	}

	fn prepare_layout(&self) {
		let computed_values = GlobalScope::get_or_init_computed_values(self.dom_node.id());
		let mut layout_info = self.layout_info_mut();
		layout_info.compute_fixed_margin(computed_values);
		layout_info.compute_fixed_padding(computed_values);
		layout_info.compute_width_and_height(computed_values);
		layout_info.compute_intrinsic(self);
	}

	/// https://www.w3.org/TR/CSS22/visudet.html#blockwidth
	fn visit_layout(&self) {
		let containing_block = self.containing_block().unwrap();
		let containing_layout = containing_block.layout_info();
		let containing_width = containing_layout.width;
		let containing_height = containing_layout.height;

		let computed_values = GlobalScope::get_or_init_computed_values(self.dom_node.id());
		let padding = BoxClass::get_padding_for_non_replaced_elements(computed_values, containing_width);
		let mut margin = BoxClass::get_margin_for_non_replaced_elements(computed_values, containing_width);
		let mut layout_info = self.layout_info_mut();

		let width = match computed_values.get_width() {
			Size::Auto => layout_info
				.intrinsic_size
				.preferred_minimum_width
				.max(containing_width - margin.left - padding.left - padding.right - margin.right),
			Size::LengthPercentage(length_percentage) => {
				let width = length_percentage.to_used_value(containing_width);
				let margin_value = containing_width - width - padding.left - padding.right;
				if margin_value <= PIXEL_ZERO {
					if *computed_values.get_margin_left() == LengthPercentageOrAuto::Auto {
						margin.left = PIXEL_ZERO;
					}
					if *computed_values.get_margin_right() == LengthPercentageOrAuto::Auto {
						margin.right = PIXEL_ZERO;
					}
				} else {
					if *computed_values.get_margin_left() == LengthPercentageOrAuto::Auto
						&& *computed_values.get_margin_right() == LengthPercentageOrAuto::Auto
					{
						margin.left = margin_value / 2.0;
						margin.right = margin_value / 2.0;
					} else if *computed_values.get_margin_left() == LengthPercentageOrAuto::Auto {
						margin.right = computed_values
							.get_margin_right()
							.to_used_value(containing_width, PIXEL_ZERO);
						margin.left = margin_value - margin.right;
					} else if *computed_values.get_margin_right() == LengthPercentageOrAuto::Auto {
						margin.left = computed_values
							.get_margin_left()
							.to_used_value(containing_width, PIXEL_ZERO);
						margin.right = margin_value - margin.left;
					}
				};
				width
			},
			Size::ExtremumLength(_) => {
				not_supported!()
			},
		};
		let height = match computed_values.get_height() {
			Size::LengthPercentage(length_percentage) => match &length_percentage.0 {
				LengthPercentage::AbsoluteLength(value) => Pixel::new(*value),
				LengthPercentage::Percentage(percentage) if containing_height != PIXEL_ZERO => {
					containing_height * percentage.to_value(&(0.0..1.0))
				},
				_ => PIXEL_ZERO,
			},
			_ => PIXEL_ZERO,
		};
		layout_info.margin = margin;
		layout_info.padding = padding;
		layout_info.width = width;
		layout_info.height = height;
		drop(layout_info);

		self.fragment.replace(self.create_fragment());
		if let Some(parent) = self.parent() {
			parent.add_child_fragment(self.fragment.clone());
		}
	}

	/// https://www.w3.org/TR/CSS22/visudet.html#normal-block
	fn revisit_layout(&self) {
		todo!()
	}

	fn class(&self) -> BoxClass {
		BoxClass::Block
	}

	fn as_block_level_box(&self) -> &BlockLevelBox {
		self
	}

	fn build_display_list(&self, builder: &mut DisplayListBuilder) {
		todo!()
	}
}
