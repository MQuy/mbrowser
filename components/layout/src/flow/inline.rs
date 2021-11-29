use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

use common::not_supported;
use css::values::computed::length::{LengthPercentage, Size};
use css::values::{Pixel, PIXEL_ZERO};
use dom::global_scope::{GlobalScope, NodeRef};

use super::boxes::{BaseBox, Box, BoxClass, SimpleBoxIterator};
use super::formatting_context::{FormattingContext, FormattingContextType};
use super::fragment::{BoxFragment, Fragment, LayoutInfo};
use crate::display_list::builder::DisplayListBuilder;

/// https://www.w3.org/TR/CSS22/visuren.html#inline-boxes
pub struct InlineLevelBox {
	dom_node: NodeRef,
	base: BaseBox,
	// if there is only one fragment -> it can have both sides.
	// else
	//  - the first one doesn't account right side for layout.
	//  - the following ones don't account left side for layout.
	fragments: RefCell<Vec<Rc<RefCell<BoxFragment>>>>,
	max_width: RefCell<Pixel>, // use for inline-level elements with IFC to create next fragments (not first one), including width + sides
}

impl InlineLevelBox {
	pub fn new(dom_node: NodeRef, formatting_context: Rc<FormattingContext>) -> Self {
		InlineLevelBox {
			dom_node,
			base: BaseBox::new(formatting_context),
			fragments: RefCell::new(Vec::with_capacity(1)),
			max_width: RefCell::new(PIXEL_ZERO),
		}
	}

	pub fn dom_node(&self) -> NodeRef {
		self.dom_node.clone()
	}

	pub fn fragments(&self) -> Ref<Vec<Rc<RefCell<BoxFragment>>>> {
		self.fragments.borrow()
	}

	pub fn fragments_mut(&self) -> RefMut<Vec<Rc<RefCell<BoxFragment>>>> {
		self.fragments.borrow_mut()
	}

	pub fn create_fragment(&self) -> BoxFragment {
		let layout_info = self.layout_info();
		let mut fragment = BoxFragment::new(self.dom_node.clone());
		fragment.padding = layout_info.padding;
		if self.fragments.borrow().len() == 0 {
			fragment.margin.left = layout_info.margin.left;
		}
		fragment.margin.right = layout_info.margin.right;
		fragment
	}

	pub fn add_fragment(&self, fragment: Rc<RefCell<BoxFragment>>) {
		self.fragments.borrow_mut().push(fragment.clone());
		self.recalculate_layout_info();
	}

	pub fn max_width(&self) -> Pixel {
		Pixel::new(self.max_width.borrow().get())
	}

	fn recalculate_layout_info(&self) {
		let mut width = PIXEL_ZERO;
		let mut height = PIXEL_ZERO;
		for fragment in self.fragments.borrow().iter() {
			width = width.max(fragment.borrow().total_width());
			height += fragment.borrow().total_height();
		}
		let mut layout_info = self.layout_info_mut();
		layout_info.width = width;
		layout_info.height = height;
	}
}

impl Box for InlineLevelBox {
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

	fn layout_info(&self) -> Ref<'_, LayoutInfo> {
		self.base.layout_info.borrow()
	}

	fn layout_info_mut(&self) -> RefMut<'_, LayoutInfo> {
		self.base.layout_info.borrow_mut()
	}

	fn add_child_fragment(&self, fragment: Rc<RefCell<dyn Fragment>>) {
		self.fragments
			.borrow_mut()
			.last()
			.unwrap()
			.borrow_mut()
			.children
			.push(fragment);
	}

	fn get_total_width(&self) -> Pixel {
		todo!()
	}

	fn get_total_height(&self) -> Pixel {
		todo!()
	}

	fn is_block_container(&self) -> bool {
		self.formatting_context_type() == FormattingContextType::BlockFormattingContext
	}

	fn prepare_layout(&self) {
		let computed_values = GlobalScope::get_or_init_computed_values(self.dom_node.id());
		let mut layout_info = self.layout_info_mut();
		layout_info.compute_fixed_margin(computed_values);
		layout_info.compute_fixed_padding(computed_values);
		if self.formatting_context_type() == FormattingContextType::BlockFormattingContext {
			layout_info.compute_width_and_height(computed_values);
		}
		layout_info.compute_intrinsic(self);
	}

	fn visit_layout(&self) {
		let parent = self.parent().unwrap();
		let establisher = parent.formatting_context().established_by();
		let (parent_current_width, parent_leftover_width, _) = BoxClass::get_parent_width(parent.clone());
		self.max_width.replace(parent_current_width + parent_leftover_width);

		let computed_values = GlobalScope::get_or_init_computed_values(self.dom_node.id());
		let containing_width = self.containing_block().unwrap().layout_info().width;
		let padding = BoxClass::get_padding_for_non_replaced_elements(computed_values, containing_width);
		let margin = BoxClass::get_margin_for_non_replaced_elements(computed_values, containing_width);
		let mut layout_info = self.layout_info_mut();
		layout_info.margin = margin;
		layout_info.padding = padding;

		match self.formatting_context_type() {
			FormattingContextType::BlockFormattingContext => {
				let width = match computed_values.get_width() {
					Size::Auto => layout_info.intrinsic_size.preferred_width.min(
						layout_info
							.intrinsic_size
							.preferred_minimum_width
							.max(containing_width - layout_info.horizontal_sides()),
					),
					Size::LengthPercentage(length_percentage) => length_percentage.to_used_value(containing_width),
					Size::ExtremumLength(_) => {
						not_supported!()
					},
				};
				let height = match computed_values.get_height() {
					Size::LengthPercentage(length_percentage) => match &length_percentage.0 {
						LengthPercentage::AbsoluteLength(value) => Pixel::new(*value),
						_ => PIXEL_ZERO,
					},
					_ => PIXEL_ZERO,
				};
				layout_info.width = width;
				layout_info.height = height;
				drop(layout_info);

				let mut fragment = self.create_fragment();
				fragment.set_width(width);
				fragment.set_bounded_width(width);
				fragment.set_height(height);
				fragment.set_bounded_height(height);

				let mut lines = establisher.lines_mut();
				let latest_line = lines.last().unwrap();

				if fragment.total_width() <= parent_leftover_width || latest_line.fragments.borrow().len() == 0 {
					fragment.set_x(parent_current_width);

					let fragment = Rc::new(RefCell::new(fragment));
					self.add_fragment(fragment.clone());
					parent.add_child_fragment(fragment.clone());

					if parent.id() == establisher.id() {
						latest_line.add_fragment(fragment.clone());
					}

					BoxClass::update_ancestors_width(fragment.clone(), establisher.clone(), self.ancestors());
				} else {
					let fragment = Rc::new(RefCell::new(fragment));
					self.add_fragment(fragment.clone());
					parent.add_child_fragment(fragment.clone());
					BoxClass::update_ancestors_with_newline(
						fragment.clone(),
						establisher.clone(),
						&mut lines,
						self.ancestors(),
					);
				}
			},
			FormattingContextType::InlineFormattingContext => {
				drop(layout_info);
				let mut fragment = self.create_fragment();
				fragment.set_bounded_width(parent_leftover_width);

				let fragment = Rc::new(RefCell::new(fragment));
				self.add_fragment(fragment.clone());
				parent.add_child_fragment(fragment.clone());

				let lines = establisher.lines();
				let latest_line = lines.last().unwrap();
				if parent.id() == establisher.id() {
					latest_line.add_fragment(fragment.clone());
				}
			},
		}
	}

	fn revisit_layout(&self) {
		todo!()
	}

	fn class(&self) -> BoxClass {
		BoxClass::Inline
	}

	fn as_inline_level_box(&self) -> &InlineLevelBox {
		self
	}

	fn build_display_list(&self, builder: &mut DisplayListBuilder) {
		todo!()
	}
}
