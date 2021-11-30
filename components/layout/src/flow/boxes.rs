use std::cell::{Ref, RefCell, RefMut};
use std::rc::{Rc, Weak};

use common::not_reached;
use css::computed_values::ComputedValues;
use css::values::{Pixel, PIXEL_ZERO};
use html5ever::{local_name, namespace_url, ns};
use uuid::Uuid;

use super::block::BlockLevelBox;
use super::formatting_context::{FormattingContext, FormattingContextType};
use super::fragment::{AnonymousFragment, Fragment, LayoutInfo, Line, Sides};
use super::inline::InlineLevelBox;
use super::text_run::TextRun;
use super::tree::VisitingContext;
use crate::display_list::builder::DisplayListBuilder;

pub trait Box {
	fn id(&self) -> Uuid;

	fn add_child(&self, child: Rc<dyn Box>);

	fn formatting_context(&self) -> Rc<FormattingContext>;

	fn set_formatting_context(&self, formatting_context: Rc<FormattingContext>);

	fn formatting_context_type(&self) -> FormattingContextType;

	fn get_first_child(&self) -> Option<Rc<dyn Box>>;

	fn get_last_child(&self) -> Option<Rc<dyn Box>>;

	fn children(&self) -> Vec<Rc<dyn Box>>;

	fn parent(&self) -> Option<Rc<dyn Box>>;

	fn set_parent(&self, value: Option<Rc<dyn Box>>);

	fn ancestors(&self) -> SimpleBoxIterator;

	fn containing_block(&self) -> Option<Rc<dyn Box>>;

	fn set_containing_block(&self, value: Option<Rc<dyn Box>>);

	fn get_total_width(&self) -> Pixel;

	fn get_total_height(&self) -> Pixel;

	fn lines(&self) -> Ref<Vec<Line>> {
		panic!("called on an element belongs to non block formatting context");
	}

	fn lines_mut(&self) -> RefMut<Vec<Line>> {
		panic!("called on an element belongs to non block formatting context");
	}

	fn layout_info(&self) -> Ref<LayoutInfo>;

	fn layout_info_mut(&self) -> RefMut<LayoutInfo>;

	fn add_child_fragment(&self, fragment: Rc<RefCell<dyn Fragment>>);

	fn is_block_container(&self) -> bool;

	fn prepare_layout(&self);

	fn visit_layout(&self);

	fn revisit_layout(&self, context: &mut VisitingContext);

	fn class(&self) -> BoxClass;

	fn as_block_level_box(&self) -> &BlockLevelBox {
		panic!("called as_block_level_box on a non-block-level box");
	}

	fn as_inline_level_box(&self) -> &InlineLevelBox {
		panic!("called as_inline_level_box on a non-inline-level box");
	}

	fn as_anonymous_box(&self) -> &AnonymousBox {
		panic!("called as_anonymous_box on a non-anonymous box");
	}

	fn as_text_run(&self) -> &TextRun {
		panic!("called as_text_run on a non text run");
	}

	fn build_display_list(&self, builder: &mut DisplayListBuilder);
}

impl PartialEq for dyn Box {
	fn eq(&self, other: &Self) -> bool {
		self.id() == other.id()
	}
}

pub struct BaseBox {
	pub id: Uuid,
	pub formatting_context: RefCell<Rc<FormattingContext>>,
	pub children: RefCell<Vec<Rc<dyn Box>>>,
	pub parent: RefCell<Option<Weak<dyn Box>>>,
	pub containing_block: RefCell<Option<Weak<dyn Box>>>,
	pub layout_info: RefCell<LayoutInfo>,
}

impl BaseBox {
	pub fn new(formatting_context: Rc<FormattingContext>) -> Self {
		BaseBox {
			id: Uuid::new_v4(),
			parent: RefCell::new(Default::default()),
			formatting_context: RefCell::new(formatting_context),
			children: RefCell::new(Default::default()),
			containing_block: RefCell::new(Default::default()),
			layout_info: RefCell::new(Default::default()),
		}
	}

	#[inline]
	pub fn add_child(&self, child: Rc<dyn Box>) {
		self.children.borrow_mut().push(child)
	}

	#[inline]
	pub fn formatting_context(&self) -> Rc<FormattingContext> {
		self.formatting_context.borrow().clone()
	}

	#[inline]
	pub fn set_formatting_context(&self, formatting_context: Rc<FormattingContext>) {
		self.formatting_context.replace(formatting_context);
	}

	#[inline]
	pub fn formatting_context_type(&self) -> FormattingContextType {
		self.formatting_context.borrow().formatting_context_type
	}

	#[inline]
	pub fn get_first_child(&self) -> Option<Rc<dyn Box>> {
		self.children.borrow().first().map(|value| value.clone())
	}

	#[inline]
	pub fn get_last_child(&self) -> Option<Rc<dyn Box>> {
		self.children.borrow().last().map(|value| value.clone())
	}

	#[inline]
	pub fn children(&self) -> Vec<Rc<dyn Box>> {
		self.children.borrow().clone()
	}

	#[inline]
	pub fn parent(&self) -> Option<Rc<dyn Box>> {
		match self.parent.borrow().as_ref() {
			Some(value) => value.upgrade(),
			None => None,
		}
	}

	#[inline]
	pub fn set_parent(&self, value: Option<Rc<dyn Box>>) {
		self.parent.replace(value.as_ref().map(|v| Rc::downgrade(v)));
	}

	#[inline]
	pub fn containing_block(&self) -> Option<Rc<dyn Box>> {
		match self.containing_block.borrow().as_ref() {
			Some(value) => value.upgrade(),
			None => None,
		}
	}

	#[inline]
	pub fn set_containing_block(&self, value: Option<Rc<dyn Box>>) {
		self.containing_block.replace(value.as_ref().map(|v| Rc::downgrade(v)));
	}
}

// Anonymous box is always anonymous block box
pub struct AnonymousBox {
	base: BaseBox,
	fragment: Rc<RefCell<AnonymousFragment>>,
	lines: RefCell<Vec<Line>>,
}

impl AnonymousBox {
	pub fn new(formatting_context: Rc<FormattingContext>) -> Self {
		AnonymousBox {
			base: BaseBox::new(formatting_context),
			fragment: Rc::new(RefCell::new(AnonymousFragment::new())),
			lines: RefCell::new(vec![]),
		}
	}

	pub fn fragment(&self) -> Ref<AnonymousFragment> {
		self.fragment.borrow()
	}

	pub fn fragment_mut(&self) -> RefMut<'_, AnonymousFragment> {
		self.fragment.borrow_mut()
	}

	pub fn create_fragment(&self) -> AnonymousFragment {
		let layout_info = self.layout_info();
		let mut fragment = AnonymousFragment::new();
		fragment.set_width(layout_info.width);
		fragment.set_bounded_width(layout_info.width);
		fragment.set_height(layout_info.height);
		fragment.set_bounded_height(layout_info.height);
		fragment
	}
}

impl Box for AnonymousBox {
	fn id(&self) -> Uuid {
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
		self.base.set_containing_block(value)
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
		self.fragment().rect.width()
	}

	fn get_total_height(&self) -> Pixel {
		self.fragment().rect.height()
	}

	fn is_block_container(&self) -> bool {
		false
	}

	fn prepare_layout(&self) {
		let mut layout_info = self.layout_info_mut();
		layout_info.compute_intrinsic(self);
	}

	fn visit_layout(&self) {
		let containing_width = self.containing_block().unwrap().layout_info().width;
		let mut layout_info = self.layout_info_mut();
		layout_info.width = containing_width;
		drop(layout_info);

		self.fragment.replace(self.create_fragment());
	}

	fn revisit_layout(&self, context: &mut VisitingContext) {
		let mut fragment = self.fragment_mut();
		fragment.set_y(context.height);

		let height = BoxClass::get_block_height(self);
		fragment.set_height(height);
		let mut layout_info = self.layout_info_mut();
		if layout_info.height == PIXEL_ZERO {
			layout_info.height = height;
		}
		BoxClass::calculate_lines(self);
		context.height += fragment.total_height();
	}

	fn class(&self) -> BoxClass {
		BoxClass::Anonymous
	}

	fn as_anonymous_box(&self) -> &AnonymousBox {
		self
	}

	fn build_display_list(&self, builder: &mut DisplayListBuilder) {}
}

#[derive(Debug, PartialEq)]
pub enum BoxClass {
	Inline,
	Block,
	Anonymous,
	TextRun,
}

impl BoxClass {
	pub fn new_with_formatting_context<F>(formatting_context_type: FormattingContextType, setup: F) -> Rc<dyn Box>
	where
		F: FnOnce(Rc<FormattingContext>) -> Rc<dyn Box>,
	{
		let formatting_context = Rc::new(FormattingContext::new(formatting_context_type));
		let level_box = setup(formatting_context.clone());
		formatting_context.set_established_by(level_box.clone());
		level_box
	}

	pub fn append_child(source: Rc<dyn Box>, child: Rc<dyn Box>) {
		let child = match source.formatting_context_type() {
			FormattingContextType::BlockFormattingContext
				if child.class() == BoxClass::Inline || child.class() == BoxClass::TextRun =>
			{
				let last_child = source.get_last_child();
				if let Some(last_child) = last_child {
					if last_child.class() == BoxClass::Anonymous {
						BoxClass::add_child(last_child, child);
						return;
					}
				}

				let anonymous_box = BoxClass::new_with_formatting_context(
					FormattingContextType::InlineFormattingContext,
					|formatting_context: Rc<FormattingContext>| Rc::new(AnonymousBox::new(formatting_context)),
				);
				child.set_formatting_context(anonymous_box.formatting_context());
				BoxClass::add_child(anonymous_box.clone(), child);
				anonymous_box
			}
			FormattingContextType::InlineFormattingContext if child.class() == BoxClass::Block => {
				not_reached!()
			},
			_ => child,
		};
		BoxClass::add_child(source, child);
	}

	pub fn add_child(source: Rc<dyn Box>, child: Rc<dyn Box>) {
		source.add_child(child.clone());
		child.set_parent(Some(source));
	}

	pub fn set_containing_box(source: Rc<dyn Box>) {
		let mut containing_block = None;
		for ancestor in source.ancestors() {
			// TODO: include box which establishes a new formatting context
			// https://www.w3.org/TR/CSS22/visudet.html#containing-block-details
			if ancestor.is_block_container() {
				containing_block = Some(ancestor);
				break;
			}
		}
		if let Some(containing_block) = containing_block {
			source.set_containing_block(Some(containing_block.clone()));
			// during constructing box tree, there might be anonymous boxes which are added (as its parent) to ensure https://www.w3.org/TR/CSS22/visuren.html#anonymous-block-level
			for ancestor in source.ancestors() {
				match ancestor.class() {
					BoxClass::Anonymous if ancestor.containing_block().is_none() => {
						ancestor.set_containing_block(Some(containing_block.clone()))
					},
					_ => break,
				}
			}
		} else {
			panic!("one of box's ancestors must be its containing box");
		}
	}

	pub fn get_padding_for_non_replaced_elements(
		computed_values: &mut ComputedValues,
		containing_width: Pixel,
	) -> Sides {
		let padding_top = computed_values.get_padding_top().to_used_value(containing_width);
		let padding_right = computed_values.get_padding_right().to_used_value(containing_width);
		let padding_bottom = computed_values.get_padding_bottom().to_used_value(containing_width);
		let padding_left = computed_values.get_padding_left().to_used_value(containing_width);
		Sides {
			top: padding_top,
			right: padding_right,
			bottom: padding_bottom,
			left: padding_left,
		}
	}

	pub fn get_margin_for_non_replaced_elements(
		computed_values: &mut ComputedValues,
		containing_width: Pixel,
	) -> Sides {
		let margin_top = computed_values
			.get_margin_top()
			.to_used_value(containing_width, PIXEL_ZERO);
		let margin_right = computed_values
			.get_margin_right()
			.to_used_value(containing_width, PIXEL_ZERO);
		let margin_bottom = computed_values
			.get_margin_bottom()
			.to_used_value(containing_width, PIXEL_ZERO);
		let margin_left = computed_values
			.get_margin_left()
			.to_used_value(containing_width, PIXEL_ZERO);
		Sides {
			top: margin_top,
			right: margin_right,
			bottom: margin_bottom,
			left: margin_left,
		}
	}

	pub fn get_parent_width(parent: Rc<dyn Box>) -> (Pixel, Pixel, Pixel) {
		match parent.class() {
			BoxClass::Inline => {
				let parent = parent.as_inline_level_box();
				let fragments = parent.as_inline_level_box().fragments();
				let latest_fragment = fragments.last().unwrap().borrow();
				let current_width = latest_fragment.width();
				let leftover_width = latest_fragment.expandable_width();
				(current_width, leftover_width, parent.max_width())
			},
			BoxClass::Block | BoxClass::Anonymous => {
				let lines = parent.lines();
				let line_width = lines.last().map_or(PIXEL_ZERO, |latest_line| latest_line.width());
				let layout_info = parent.layout_info();
				(line_width, layout_info.width - line_width, layout_info.width)
			},
			BoxClass::TextRun => not_reached!(),
		}
	}

	pub fn update_ancestors_width(
		fragment: Rc<RefCell<dyn Fragment>>,
		establisher: Rc<dyn Box>,
		ancestors: SimpleBoxIterator,
	) {
		let mut child_fragment = fragment;
		for ancestor in ancestors {
			if ancestor.id() == establisher.id() {
				break;
			}
			let inline_box = ancestor.as_inline_level_box();
			let fragments = inline_box.fragments();
			let fragment = fragments.last().unwrap();
			let fragment_width = fragment.borrow().width();
			fragment
				.borrow_mut()
				.set_width(fragment_width + child_fragment.borrow().total_width());

			child_fragment = fragment.clone();
		}
	}

	pub fn update_ancestors_with_newline(
		fragment: Rc<RefCell<dyn Fragment>>,
		establisher: Rc<dyn Box>,
		lines: &mut RefMut<Vec<Line>>,
		ancestors: SimpleBoxIterator,
	) {
		let latest_line = Line::new();
		let mut child_fragment = fragment;
		for ancestor in ancestors {
			if ancestor.id() == establisher.id() {
				latest_line.add_fragment(child_fragment.clone());
				break;
			}

			let inline_box = ancestor.as_inline_level_box();
			let parent = inline_box.parent().unwrap();

			let mut fragment = inline_box.create_fragment();
			fragment.set_width(child_fragment.borrow().width() + child_fragment.borrow().right_sides());
			fragment.set_bounded_width(inline_box.max_width() - inline_box.layout_info().right_sides());

			let fragment = Rc::new(RefCell::new(fragment));
			inline_box.add_fragment(fragment.clone());
			parent.add_child_fragment(fragment.clone());

			child_fragment = fragment;
		}
		lines.push(latest_line);
	}

	pub fn calculate_lines(source: &dyn Box) {
		let mut height = PIXEL_ZERO;
		for line in source.lines().iter() {
			line.set_y(height);
			height += line.height();
		}
	}

	pub fn get_block_height(source: &dyn Box) -> Pixel {
		match source.formatting_context_type() {
			FormattingContextType::BlockFormattingContext => {
				let mut height = PIXEL_ZERO;
				for child in source.children() {
					height += child.layout_info().total_height();
				}
				height
			},
			FormattingContextType::InlineFormattingContext => {
				let mut height = PIXEL_ZERO;
				for line in source.lines().iter() {
					height += line.height();
				}
				height
			},
		}
	}
}

pub struct SimpleBoxIterator<'a> {
	current: Option<Rc<dyn Box>>,
	next_node: &'a dyn Fn(&Rc<dyn Box>) -> Option<Rc<dyn Box>>,
}

impl<'a> SimpleBoxIterator<'a> {
	pub fn new(current: Option<Rc<dyn Box>>, next_node: &'a dyn Fn(&Rc<dyn Box>) -> Option<Rc<dyn Box>>) -> Self {
		SimpleBoxIterator { current, next_node }
	}
}

impl<'a> Iterator for SimpleBoxIterator<'a> {
	type Item = Rc<dyn Box>;

	fn next(&mut self) -> Option<Self::Item> {
		let current = self.current.take();
		self.current = current.as_ref().and_then(|c| (self.next_node)(c));
		current
	}
}
