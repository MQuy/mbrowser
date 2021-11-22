use std::cell::{Ref, RefCell, RefMut};
use std::rc::{Rc, Weak};

use common::not_reached;
use css::values::{CSSPixel, Pixel};
use dom::characterdata::CharacterData;
use dom::global_scope::{GlobalScope, NodeRef};
use dom::inheritance::Castable;
use euclid::Rect;
use regex::Regex;
use uuid::Uuid;

use super::boxes::{Box, BoxClass, SimpleBoxIterator};
use super::formatting_context::{FormattingContext, FormattingContextType};
use super::fragment::{Fragment, IntrinsicSize, LayoutInfo};
use crate::display_list::builder::DisplayListBuilder;
use crate::text::TextUI;

/// https://www.w3.org/TR/CSS22/visuren.html#inline-boxes
pub struct TextRun {
	id: Uuid,
	dom_node: NodeRef,
	parent: RefCell<Option<Weak<dyn Box>>>,
	containing_block: RefCell<Option<Weak<dyn Box>>>,
	formatting_context: RefCell<Rc<FormattingContext>>,
	fragments: RefCell<Vec<Fragment>>,
	intrinsic_size: RefCell<IntrinsicSize>,
}

impl TextRun {
	pub fn new(dom_node: NodeRef, formatting_context: Rc<FormattingContext>) -> Self {
		TextRun {
			dom_node,
			id: Uuid::new_v4(),
			parent: RefCell::new(None),
			containing_block: RefCell::new(None),
			formatting_context: RefCell::new(formatting_context),
			fragments: RefCell::new(Vec::with_capacity(1)),
			intrinsic_size: RefCell::new(Default::default()),
		}
	}

	pub fn dom_node(&self) -> NodeRef {
		self.dom_node.clone()
	}
}

impl Box for TextRun {
	fn id(&self) -> uuid::Uuid {
		self.id
	}

	fn add_child(&self, child: Rc<dyn Box>) {
		not_reached!()
	}

	fn formatting_context(&self) -> Rc<FormattingContext> {
		self.formatting_context.borrow().clone()
	}

	fn set_formatting_context(&self, formatting_context: Rc<FormattingContext>) {
		self.formatting_context.replace(formatting_context);
	}

	fn formatting_context_type(&self) -> FormattingContextType {
		self.formatting_context.borrow().formatting_context_type
	}

	fn get_first_child(&self) -> Option<Rc<dyn Box>> {
		not_reached!()
	}

	fn get_last_child(&self) -> Option<Rc<dyn Box>> {
		not_reached!()
	}

	fn children(&self) -> Vec<Rc<dyn Box>> {
		not_reached!()
	}

	fn parent(&self) -> Option<Rc<dyn Box>> {
		match self.parent.borrow().as_ref() {
			Some(value) => value.upgrade(),
			None => None,
		}
	}

	fn set_parent(&self, value: Option<Rc<dyn Box>>) {
		self.parent
			.replace(value.as_ref().map(|v| Rc::downgrade(v)));
	}

	fn ancestors(&self) -> SimpleBoxIterator {
		SimpleBoxIterator::new(self.parent(), &|n: &Rc<dyn Box>| n.parent())
	}

	fn containing_block(&self) -> Option<Rc<dyn Box>> {
		match self.containing_block.borrow().as_ref() {
			Some(value) => value.upgrade(),
			None => None,
		}
	}

	fn set_containing_block(&self, value: Option<Rc<dyn Box>>) {
		self.containing_block
			.replace(value.as_ref().map(|v| Rc::downgrade(v)));
	}

	fn layout_info(&self) -> Ref<'_, LayoutInfo> {
		not_reached!()
	}

	fn layout_info_mut(&self) -> RefMut<'_, LayoutInfo> {
		not_reached!()
	}

	fn content_rect(&self) -> Rect<Pixel, CSSPixel> {
		todo!()
	}

	fn get_total_width(&self) -> Pixel {
		todo!()
	}

	fn get_total_height(&self) -> Pixel {
		todo!()
	}

	fn is_block_container(&self) -> bool {
		false
	}

	fn prepare_layout(&self) {
		let text_ui = TextUI::new();
		let content = self.dom_node.downcast::<CharacterData>().data();
		let computed_values =
			GlobalScope::get_or_init_computed_values(self.dom_node.parent_node().unwrap().id());
		let family_names = computed_values.get_font_families();
		let font_size = computed_values.get_font_size();
		let (width, _) = text_ui.measure_size(content.as_str(), family_names, font_size);
		let mut intrinsic_size = self.intrinsic_size.borrow_mut();
		intrinsic_size.preferred_width = Pixel::new(width);

		let regex = Regex::new(r"\s").unwrap();
		for word in regex.split(content.as_str()) {
			let (width, _) = text_ui.measure_size(word, family_names, font_size);
			intrinsic_size.preferred_minimum_width = intrinsic_size
				.preferred_minimum_width
				.max(Pixel::new(width));
		}
	}

	fn visit_layout(&self) {
		todo!()
	}

	fn revisit_layout(&self) {
		todo!()
	}

	fn class(&self) -> BoxClass {
		BoxClass::TextRun
	}

	fn as_text_run(&self) -> &TextRun {
		self
	}

	fn build_display_list(&self, builder: &mut DisplayListBuilder) {
		todo!()
	}
}
