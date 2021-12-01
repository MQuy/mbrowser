use std::cell::{Ref, RefCell, RefMut};
use std::rc::{Rc, Weak};

use common::not_reached;
use css::values::{Pixel, PIXEL_ZERO};
use dom::characterdata::CharacterData;
use dom::global_scope::{GlobalScope, NodeRef};
use dom::inheritance::Castable;
use regex::Regex;
use uuid::Uuid;

use super::boxes::{Box, BoxClass, SimpleBoxIterator};
use super::formatting_context::{FormattingContext, FormattingContextType};
use super::fragment::{Fragment, LayoutInfo, Line, TextFragment};
use super::tree::VisitingContext;
use crate::text::TextUI;

/// https://www.w3.org/TR/CSS22/visuren.html#inline-boxes
pub struct TextRun {
	id: Uuid,
	dom_node: NodeRef,
	parent: RefCell<Option<Weak<dyn Box>>>,
	containing_block: RefCell<Option<Weak<dyn Box>>>,
	formatting_context: RefCell<Rc<FormattingContext>>,
	fragments: RefCell<Vec<Rc<RefCell<TextFragment>>>>,
	layout_info: RefCell<LayoutInfo>,
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
			layout_info: RefCell::new(Default::default()),
		}
	}

	pub fn dom_node(&self) -> NodeRef {
		self.dom_node.clone()
	}

	pub fn add_fragment(&self, fragment: Rc<RefCell<TextFragment>>) {
		self.fragments.borrow_mut().push(fragment)
	}

	pub fn fragments(&self) -> Ref<Vec<Rc<RefCell<TextFragment>>>> {
		self.fragments.borrow()
	}

	pub fn create_fragment(&self, content: String) -> TextFragment {
		TextFragment::new(self.dom_node(), content)
	}

	fn split_paragraph(
		&self,
		width: Pixel,
		height: Pixel,
		parts: &Vec<&str>,
		parent_current_width: Pixel,
		in_current_line: bool,
		parent: Rc<dyn Box>,
		establisher: Rc<dyn Box>,
	) {
		let mut fragment = self.create_fragment(parts.join(""));
		fragment.set_width(width);
		fragment.set_height(height);

		if in_current_line {
			fragment.set_x(parent_current_width);
			let fragment = Rc::new(RefCell::new(fragment));
			self.add_fragment(fragment.clone());
			parent.add_child_fragment(fragment.clone());

			let mut lines = establisher.lines_mut();
			if lines.len() == 0 {
				lines.push(Line::new());
			}
			let latest_line = lines.last().unwrap();
			if parent.id() == establisher.id() {
				latest_line.add_fragment(fragment.clone());
			}
			BoxClass::update_ancestors_width(fragment.clone(), establisher.clone(), self.ancestors());
		} else {
			let mut lines = establisher.lines_mut();
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
	}
}

impl Box for TextRun {
	fn id(&self) -> uuid::Uuid {
		self.id
	}

	fn add_child(&self, _child: Rc<dyn Box>) {
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
		None
	}

	fn get_last_child(&self) -> Option<Rc<dyn Box>> {
		None
	}

	fn children(&self) -> Vec<Rc<dyn Box>> {
		vec![]
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
		self.layout_info.borrow()
	}

	fn layout_info_mut(&self) -> RefMut<'_, LayoutInfo> {
		self.layout_info.borrow_mut()
	}

	fn add_child_fragment(&self, _fragment: Rc<RefCell<dyn super::fragment::Fragment>>) {
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
		let (width, height) = text_ui.measure_size(content.as_str(), family_names, font_size);
		let mut layout_info = self.layout_info.borrow_mut();
		layout_info.intrinsic_size.preferred_width = Pixel::new(width);
		layout_info.intrinsic_size.preferred_height = Pixel::new(height);

		let regex = Regex::new(r"\s").unwrap();
		for word in regex.split(content.as_str()) {
			let (width, _) = text_ui.measure_size(word, family_names, font_size);
			layout_info.intrinsic_size.preferred_minimum_width = layout_info
				.intrinsic_size
				.preferred_minimum_width
				.max(Pixel::new(width));
		}
	}

	fn visit_layout(&self) {
		let text_ui = TextUI::new();
		let parent = self.parent().unwrap();
		let (parent_current_width, parent_leftover_width, parent_max_width) =
			BoxClass::get_parent_width(parent.clone());
		let layout_info = self.layout_info.borrow();

		let establisher = parent.formatting_context().established_by();

		let content = self.dom_node.downcast::<CharacterData>().data();
		let computed_values = GlobalScope::get_or_init_computed_values(self.dom_node.parent_node().unwrap().id());
		let family_names = computed_values.get_font_families();
		let font_size = computed_values.get_font_size();

		if layout_info.intrinsic_size.preferred_width <= parent_leftover_width {
			let mut fragment = self.create_fragment(content.to_string());
			fragment.set_width(layout_info.intrinsic_size.preferred_width);
			fragment.set_height(layout_info.intrinsic_size.preferred_height);
			fragment.set_x(parent_current_width);

			let fragment = Rc::new(RefCell::new(fragment));
			self.add_fragment(fragment.clone());
			parent.add_child_fragment(fragment.clone());

			let mut lines = establisher.lines_mut();
			if lines.len() == 0 {
				lines.push(Line::new());
			}
			let latest_line = lines.last().unwrap();
			if parent.id() == establisher.id() {
				latest_line.add_fragment(fragment.clone());
			}
			BoxClass::update_ancestors_width(fragment.clone(), establisher.clone(), self.ancestors());
		} else {
			let mut max_width = parent_leftover_width;
			let mut width = PIXEL_ZERO;
			let mut height = PIXEL_ZERO;
			let mut parts: Vec<&str> = vec![];
			let mut in_current_line = true;

			let regex = Regex::new(r"\s").unwrap();
			for word in split_keep(&regex, content.as_ref()) {
				let bounds = text_ui.measure_size(word, family_names, font_size);
				let word_width = Pixel::new(bounds.0);

				if width + word_width <= max_width {
					width += word_width;
					height = height.max(Pixel::new(bounds.1));
				} else {
					self.split_paragraph(
						width,
						height,
						&parts,
						parent_current_width,
						in_current_line,
						parent.clone(),
						establisher.clone(),
					);

					max_width = parent_max_width;
					width = word_width;
					height = Pixel::new(bounds.1);
					parts.clear();
					in_current_line = false;
				}
				parts.push(word);
			}
			self.split_paragraph(
				width,
				height,
				&parts,
				parent_current_width,
				in_current_line,
				parent,
				establisher,
			)
		}
	}

	fn revisit_layout(&self, context: &mut VisitingContext) {
		let mut height = PIXEL_ZERO;
		for fragment in self.fragments.borrow().iter() {
			height += fragment.borrow_mut().total_height();
		}
		let mut layout_info = self.layout_info_mut();
		layout_info.height = height;
	}

	fn class(&self) -> BoxClass {
		BoxClass::TextRun
	}

	fn as_text_run(&self) -> &TextRun {
		self
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
