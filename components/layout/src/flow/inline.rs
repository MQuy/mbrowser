use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

use css::values::{CSSPixel, Pixel};
use dom::global_scope::{GlobalScope, NodeRef};
use euclid::Rect;

use super::boxes::{BaseBox, Box, BoxClass, SimpleBoxIterator};
use super::formatting_context::{FormattingContext, FormattingContextType};
use super::fragment::{BoxFragment, LayoutInfo};
use crate::display_list::builder::DisplayListBuilder;

/// https://www.w3.org/TR/CSS22/visuren.html#inline-boxes
pub struct InlineLevelBox {
	dom_node: NodeRef,
	base: BaseBox,
	fragments: RefCell<Vec<Rc<BoxFragment>>>,
}

impl InlineLevelBox {
	pub fn new(dom_node: NodeRef, formatting_context: Rc<FormattingContext>) -> Self {
		InlineLevelBox {
			dom_node,
			base: BaseBox::new(formatting_context),
			fragments: RefCell::new(Vec::with_capacity(1)),
		}
	}

	pub fn dom_node(&self) -> NodeRef {
		self.dom_node.clone()
	}

	pub fn fragments(&self) -> Ref<'_, Vec<Rc<BoxFragment>>> {
		self.fragments.borrow()
	}

	pub fn fragments_mut(&self) -> RefMut<'_, Vec<Rc<BoxFragment>>> {
		self.fragments.borrow_mut()
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

	fn content_rect(&self) -> Rect<Pixel, CSSPixel> {
		todo!()
	}

	fn layout_info(&self) -> Ref<'_, LayoutInfo> {
		self.base.layout_info.borrow()
	}

	fn layout_info_mut(&self) -> RefMut<'_, LayoutInfo> {
		self.base.layout_info.borrow_mut()
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

	/// https://www.w3.org/TR/CSS22/visudet.html#inlineblock-width
	fn visit_layout(&self) {
		todo!()
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
