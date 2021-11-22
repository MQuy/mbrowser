use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

use common::not_supported;
use css::values::computed::length::{LengthPercentage, LengthPercentageOrAuto, Size};
use css::values::{Pixel, PIXEL_ZERO};
use dom::global_scope::{GlobalScope, NodeRef};
use euclid::Rect;

use super::boxes::{BaseBox, Box, BoxClass, SimpleBoxIterator};
use super::formatting_context::{FormattingContext, FormattingContextType};
use super::fragment::{BoxFragment, Fragment, LayoutInfo, Line};
use crate::display_list::builder::DisplayListBuilder;

/// https://www.w3.org/TR/CSS22/visuren.html#block-boxes
pub struct BlockLevelBox {
	dom_node: NodeRef,
	base: BaseBox,
	fragment: RefCell<BoxFragment>,
	lines: RefCell<Vec<Line>>,
}

impl BlockLevelBox {
	pub fn new(dom_node: NodeRef, formatting_context: Rc<FormattingContext>) -> Self {
		BlockLevelBox {
			dom_node: dom_node.clone(),
			base: BaseBox::new(formatting_context),
			fragment: RefCell::new(BoxFragment::new(dom_node)),
			lines: RefCell::new(Default::default()),
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

	pub fn lines(&self) -> Ref<'_, Vec<Line>> {
		self.lines.borrow()
	}

	pub fn lines_mut(&self) -> RefMut<'_, Vec<Line>> {
		self.lines.borrow_mut()
	}

	pub fn append_fragment(&self, fragment: Rc<Fragment>) {
		let mut lines = self.lines.borrow_mut();
		let line = lines.last_mut().unwrap();
		line.fragments.push(fragment.clone());
		line.bounds.size.width += fragment.total_width();
	}

	pub fn create_newline(&self) {
		self.lines.borrow_mut().push(Line::new());
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

	fn content_rect(&self) -> Rect<Pixel, css::values::CSSPixel> {
		self.fragment().rect
	}

	fn layout_info(&self) -> Ref<'_, LayoutInfo> {
		self.base.layout_info.borrow()
	}

	fn layout_info_mut(&self) -> RefMut<'_, LayoutInfo> {
		self.base.layout_info.borrow_mut()
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
		let computed_values = GlobalScope::get_or_init_computed_values(self.dom_node.id());
		let containing_block = self.containing_block().unwrap();
		let containing_width = containing_block.content_rect().width();
		let containing_height = containing_block.content_rect().width();
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
					.get_margin_right()
					.to_used_value(containing_width, PIXEL_ZERO);
				PIXEL_ZERO.max(
					containing_width - margin_left - padding_left - padding_right - margin_right,
				)
			},
			Size::LengthPercentage(length_percentage) => {
				let width = length_percentage.to_used_value(containing_width);
				let margin = containing_width - width - padding_left - padding_right;
				if margin <= PIXEL_ZERO {
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
		let mut fragment = self.fragment_mut();
		fragment.padding.left = padding_left;
		fragment.padding.right = padding_right;
		fragment.margin.left = margin_left;
		fragment.margin.right = margin_right;
		fragment.rect.origin.x = margin_left + padding_left;
		fragment.rect.size.width = width;
		fragment.rect.size.height = height;
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
