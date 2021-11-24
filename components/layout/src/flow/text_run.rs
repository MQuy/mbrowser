use std::cell::{Ref, RefCell, RefMut};
use std::rc::{Rc, Weak};

use common::not_reached;
use css::values::{CSSPixel, Pixel, PIXEL_ZERO};
use dom::characterdata::CharacterData;
use dom::global_scope::{GlobalScope, NodeRef};
use dom::inheritance::Castable;
use euclid::Rect;
use regex::Regex;
use uuid::Uuid;

use super::boxes::{Box, BoxClass, SimpleBoxIterator};
use super::formatting_context::{FormattingContext, FormattingContextType};
use super::fragment::{Fragment, IntrinsicSize, LayoutInfo, TextFragment};
use crate::display_list::builder::DisplayListBuilder;
use crate::text::TextUI;

/// https://www.w3.org/TR/CSS22/visuren.html#inline-boxes
pub struct TextRun {
	id: Uuid,
	dom_node: NodeRef,
	parent: RefCell<Option<Weak<dyn Box>>>,
	containing_block: RefCell<Option<Weak<dyn Box>>>,
	formatting_context: RefCell<Rc<FormattingContext>>,
	fragments: RefCell<Vec<Rc<RefCell<TextFragment>>>>,
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
		self.parent.replace(value.as_ref().map(|v| Rc::downgrade(v)));
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
		self.containing_block.replace(value.as_ref().map(|v| Rc::downgrade(v)));
	}

	fn layout_info(&self) -> Ref<'_, LayoutInfo> {
		not_reached!()
	}

	fn layout_info_mut(&self) -> RefMut<'_, LayoutInfo> {
		not_reached!()
	}

	fn add_child_fragment(&self, fragment: Rc<RefCell<dyn super::fragment::Fragment>>) {
		not_reached!()
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
		let computed_values = GlobalScope::get_or_init_computed_values(self.dom_node.parent_node().unwrap().id());
		let family_names = computed_values.get_font_families();
		let font_size = computed_values.get_font_size();
		let (width, _) = text_ui.measure_size(content.as_str(), family_names, font_size);
		let mut intrinsic_size = self.intrinsic_size.borrow_mut();
		intrinsic_size.preferred_width = Pixel::new(width);

		let regex = Regex::new(r"\s").unwrap();
		for word in regex.split(content.as_str()) {
			let (width, _) = text_ui.measure_size(word, family_names, font_size);
			intrinsic_size.preferred_minimum_width = intrinsic_size.preferred_minimum_width.max(Pixel::new(width));
		}
	}

	fn visit_layout(&self) {
		let text_ui = TextUI::new();
		let parent = self.parent().unwrap();
		let (parent_current_width, parent_leftover_width, parent_max_width) = BoxClass::parent_widths(parent.clone());
		let intrinsic_size = self.intrinsic_size.borrow();

		let establisher = parent.formatting_context().established_by();
		let lines = establisher.lines();
		let latest_line = lines.last().unwrap();

		let content = self.dom_node.downcast::<CharacterData>().data();
		let computed_values = GlobalScope::get_or_init_computed_values(self.dom_node.parent_node().unwrap().id());
		let family_names = computed_values.get_font_families();
		let font_size = computed_values.get_font_size();
		if intrinsic_size.preferred_width <= parent_leftover_width {
			let mut fragment = TextFragment::new(self.dom_node(), content.to_string());
			fragment.set_width(intrinsic_size.preferred_width);
			fragment.set_x(parent_current_width);

			let fragment = Rc::new(RefCell::new(fragment));
			BoxClass::update_ancestors_width(
				parent,
				fragment.clone(),
				establisher.clone(),
				latest_line,
				self.ancestors(),
			);
			self.fragments.borrow_mut().push(fragment);
		} else {
			let regex = Regex::new(r"\s").unwrap();
			let mut max_width = parent_leftover_width;
			let mut width = PIXEL_ZERO;
			let mut parts: Vec<&str> = vec![];
			let mut in_current_line = true;
			for word in split_keep(&regex, content.as_ref()) {
				let word_width = Pixel::new(text_ui.measure_size(word, family_names, font_size).0);
				if width + word_width <= max_width {
					width += word_width;
				} else {
					max_width = parent_max_width;
					width = word_width;
					parts.clear();

					if in_current_line {
						let mut fragment = TextFragment::new(self.dom_node(), parts.join(""));
						fragment.set_x(parent_current_width);
						let fragment = Rc::new(RefCell::new(fragment));
						BoxClass::update_ancestors_width(
							parent.clone(),
							fragment.clone(),
							establisher.clone(),
							latest_line,
							self.ancestors(),
						);
						self.fragments.borrow_mut().push(fragment);
					} else {
						let fragment = Rc::new(RefCell::new(TextFragment::new(self.dom_node(), parts.join(""))));
						BoxClass::update_ancestors_newline(
							parent.clone(),
							fragment.clone(),
							establisher.clone(),
							self.ancestors(),
						);
						self.fragments.borrow_mut().push(fragment);
					}
					in_current_line = false;
				}
				parts.push(word);
			}
		}
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

fn split_keep<'a>(r: &Regex, text: &'a str) -> Vec<&'a str> {
	let mut result = Vec::new();
	let mut last = 0;
	for (index, matched) in text.match_indices(r) {
		if last != index {
			result.push(&text[last..index]);
		}
		result.push(matched);
		last = index + matched.len();
	}
	if last < text.len() {
		result.push(&text[last..]);
	}
	result
}
