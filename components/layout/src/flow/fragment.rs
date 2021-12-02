use std::cell::{Ref, RefCell};
use std::rc::Rc;

use css::computed_values::ComputedValues;
use css::values::{CSSPixel, Pixel, PIXEL_ZERO};
use dom::global_scope::{GlobalScope, NodeRef};
use euclid::{Point2D, Rect, Size2D};

use super::boxes::Box;
use super::formatting_context::FormattingContextType;
use crate::display_list::builder::{BuilderContext, DisplayListBuilder};

pub struct Line {
	pub fragments: RefCell<Vec<Rc<RefCell<dyn Fragment>>>>, // BoxFragment or TextFragment
	pub bounds: RefCell<Rect<Pixel, CSSPixel>>,
}

impl Line {
	pub fn new() -> Self {
		Line {
			fragments: Default::default(),
			bounds: Default::default(),
		}
	}

	pub fn width(&self) -> Pixel {
		let mut width = PIXEL_ZERO;
		for fragment in self.fragments.borrow().iter() {
			width += fragment.borrow().total_width();
		}
		width
	}

	pub fn height(&self) -> Pixel {
		let mut height = PIXEL_ZERO;
		for fragment in self.fragments.borrow().iter() {
			height = height.max(fragment.borrow().total_height());
		}
		height
	}

	pub fn y(&self) -> Pixel {
		self.bounds.borrow().origin.y
	}

	pub fn set_y(&self, value: Pixel) {
		self.bounds.borrow_mut().origin.y = value;
	}

	pub fn add_fragment(&self, fragment: Rc<RefCell<dyn Fragment>>) {
		self.fragments.borrow_mut().push(fragment.clone());
	}

	pub fn fragments(&self) -> Ref<Vec<Rc<RefCell<dyn Fragment>>>> {
		self.fragments.borrow()
	}
}

#[derive(Clone, Copy, Debug)]
pub struct Sides {
	pub top: Pixel,
	pub right: Pixel,
	pub bottom: Pixel,
	pub left: Pixel,
}

impl Default for Sides {
	fn default() -> Self {
		Self {
			top: PIXEL_ZERO,
			right: PIXEL_ZERO,
			bottom: PIXEL_ZERO,
			left: PIXEL_ZERO,
		}
	}
}

pub struct IntrinsicSize {
	pub preferred_width: Pixel,
	pub preferred_height: Pixel,
	pub preferred_minimum_width: Pixel,
}

impl Default for IntrinsicSize {
	fn default() -> Self {
		Self {
			preferred_width: PIXEL_ZERO,
			preferred_minimum_width: PIXEL_ZERO,
			preferred_height: PIXEL_ZERO,
		}
	}
}

pub struct LayoutInfo {
	pub width: Pixel,
	pub height: Pixel,
	pub margin: Sides,
	pub padding: Sides,
	pub intrinsic_size: IntrinsicSize,
}

impl Default for LayoutInfo {
	fn default() -> Self {
		Self {
			width: PIXEL_ZERO,
			height: PIXEL_ZERO,
			margin: Default::default(),
			padding: Default::default(),
			intrinsic_size: Default::default(),
		}
	}
}

impl LayoutInfo {
	pub fn total_width(&self) -> Pixel {
		self.margin.left + self.padding.left + self.width + self.padding.right + self.margin.right
	}

	pub fn total_height(&self) -> Pixel {
		self.margin.top + self.padding.top + self.height + self.padding.bottom + self.margin.bottom
	}

	pub fn set_padding(&mut self, top: Pixel, right: Pixel, bottom: Pixel, left: Pixel) {
		self.padding.top = top;
		self.padding.right = right;
		self.padding.bottom = bottom;
		self.padding.left = left;
	}

	pub fn set_margin(&mut self, top: Pixel, right: Pixel, bottom: Pixel, left: Pixel) {
		self.margin.top = top;
		self.margin.right = right;
		self.margin.bottom = bottom;
		self.margin.left = left;
	}

	pub fn horizontal_sides(&self) -> Pixel {
		self.margin.left + self.padding.left + self.padding.right + self.margin.right
	}

	pub fn right_sides(&self) -> Pixel {
		self.padding.right + self.margin.right
	}

	pub fn vertical_sides(&self) -> Pixel {
		self.margin.top + self.padding.top + self.padding.bottom + self.padding.bottom
	}

	pub fn compute_fixed_margin(&mut self, computed_values: &mut ComputedValues) {
		if let Some(margin_top) = computed_values.get_margin_top().to_fixed_used_value() {
			self.margin.top = margin_top;
		}
		if let Some(margin_bottom) = computed_values.get_margin_bottom().to_fixed_used_value() {
			self.margin.bottom = margin_bottom;
		}
		if let Some(margin_left) = computed_values.get_margin_left().to_fixed_used_value() {
			self.margin.left = margin_left;
		}
		if let Some(margin_right) = computed_values.get_margin_right().to_fixed_used_value() {
			self.margin.right = margin_right;
		}
	}

	pub fn compute_fixed_padding(&mut self, computed_values: &mut ComputedValues) {
		if let Some(padding_top) = computed_values.get_padding_top().to_fixed_used_value() {
			self.padding.top = padding_top;
		}
		if let Some(padding_bottom) = computed_values.get_padding_bottom().to_fixed_used_value() {
			self.padding.bottom = padding_bottom;
		}
		if let Some(padding_left) = computed_values.get_padding_left().to_fixed_used_value() {
			self.padding.left = padding_left;
		}
		if let Some(padding_right) = computed_values.get_padding_right().to_fixed_used_value() {
			self.padding.right = padding_right;
		}
	}

	pub fn compute_width_and_height(&mut self, computed_values: &mut ComputedValues) {
		if let Some(width) = computed_values.get_width().to_fixed_used_value() {
			self.width = width;
			self.intrinsic_size.preferred_minimum_width = width;
			self.intrinsic_size.preferred_width = width;
		}
		if let Some(height) = computed_values.get_height().to_fixed_used_value() {
			self.height = height;
		}
	}

	pub fn compute_intrinsic(&mut self, node: &dyn Box) {
		let mut preferred_minimum_width = PIXEL_ZERO;
		let mut preferred_width = PIXEL_ZERO;
		match node.formatting_context_type() {
			FormattingContextType::BlockFormattingContext => {
				for child in node.children() {
					let child_layout_info = child.layout_info();
					preferred_minimum_width = preferred_minimum_width.max(
						child_layout_info.intrinsic_size.preferred_minimum_width + child_layout_info.horizontal_sides(),
					);
					preferred_width = preferred_width.max(child_layout_info.intrinsic_size.preferred_width)
						+ child_layout_info.horizontal_sides();
				}
			},
			FormattingContextType::InlineFormattingContext => {
				for child in node.children() {
					let child_layout_info = child.layout_info();
					preferred_minimum_width = preferred_minimum_width.max(
						child_layout_info.intrinsic_size.preferred_minimum_width + child_layout_info.horizontal_sides(),
					);
					preferred_width +=
						child_layout_info.intrinsic_size.preferred_width + child_layout_info.horizontal_sides();
				}
			},
		};
		self.intrinsic_size.preferred_minimum_width = preferred_minimum_width;
		self.intrinsic_size.preferred_width = preferred_width;
	}
}

pub trait Fragment {
	fn width(&self) -> Pixel;

	fn height(&self) -> Pixel;

	fn total_width(&self) -> Pixel;

	fn total_height(&self) -> Pixel;

	fn right_sides(&self) -> Pixel;

	fn x(&self) -> Pixel;

	fn y(&self) -> Pixel;

	fn rect_x(&self) -> Pixel;

	fn rect_y(&self) -> Pixel;

	fn class(&self) -> FragmentClass;

	fn as_box_fragment(&self) -> &BoxFragment {
		panic!("called as_box_fragment on a non box fragment");
	}

	fn as_text_fragment(&self) -> &TextFragment {
		panic!("called as_text_fragment on a non text fragment");
	}

	fn as_anonymous_fragment(&self) -> &AnonymousFragment {
		panic!("called as_anonymous_fragment on a non anonymous fragment");
	}

	fn build_display_list(&self, builder: &mut DisplayListBuilder, context: &mut BuilderContext);
}

pub enum FragmentClass {
	BoxFragment,
	TextFragment,
	AnonymousFragment,
}

pub struct BoxFragment {
	pub dom_node: NodeRef,
	pub padding: Sides,
	pub margin: Sides,
	pub rect: Rect<Pixel, CSSPixel>,
	pub bounds: Size2D<Pixel, CSSPixel>,
	pub children: Vec<Rc<RefCell<dyn Fragment>>>,
	pub lines: Rc<RefCell<Vec<Line>>>,
}

impl Fragment for BoxFragment {
	fn total_width(&self) -> Pixel {
		self.margin.left + self.padding.left + self.rect.width() + self.padding.right + self.margin.right
	}

	fn total_height(&self) -> Pixel {
		self.margin.top + self.padding.top + self.rect.height() + self.padding.bottom + self.margin.bottom
	}

	fn width(&self) -> Pixel {
		self.rect.width()
	}

	fn height(&self) -> Pixel {
		self.rect.height()
	}

	fn right_sides(&self) -> Pixel {
		self.padding.right + self.margin.right
	}

	fn x(&self) -> Pixel {
		self.rect.origin.x
	}

	fn y(&self) -> Pixel {
		self.rect.origin.y
	}

	fn rect_x(&self) -> Pixel {
		self.x() + self.margin.left + self.padding.left
	}

	fn rect_y(&self) -> Pixel {
		self.y() + self.margin.top + self.padding.top
	}

	fn class(&self) -> FragmentClass {
		FragmentClass::BoxFragment
	}

	fn as_box_fragment(&self) -> &BoxFragment {
		self
	}

	fn build_display_list(&self, builder: &mut DisplayListBuilder, context: &mut BuilderContext) {
		let computed_values = GlobalScope::get_or_init_computed_values(self.dom_node.id());
		builder.push_rect(
			Rect::new(
				Point2D::new(
					context.x + self.rect.origin.x + self.margin.left + self.padding.left,
					context.y + self.rect.origin.y + self.margin.top + self.padding.top,
				),
				self.rect.size,
			),
			computed_values.get_background_color().clone(),
		);
	}
}

impl BoxFragment {
	pub fn new(dom_node: NodeRef, lines: Rc<RefCell<Vec<Line>>>) -> Self {
		BoxFragment {
			dom_node,
			lines,
			rect: Default::default(),
			padding: Default::default(),
			margin: Default::default(),
			bounds: Default::default(),
			children: Default::default(),
		}
	}

	pub fn expandable_width(&self) -> Pixel {
		(self.bounds.width - self.rect.width()).max(PIXEL_ZERO)
	}

	#[inline]
	pub fn set_width(&mut self, value: Pixel) {
		self.rect.size.width = value;
	}

	#[inline]
	pub fn set_height(&mut self, value: Pixel) {
		self.rect.size.height = value;
	}

	#[inline]
	pub fn set_bounded_width(&mut self, value: Pixel) {
		self.bounds.width = value;
	}

	#[inline]
	pub fn set_bounded_height(&mut self, value: Pixel) {
		self.bounds.height = value;
	}

	#[inline]
	pub fn set_x(&mut self, value: Pixel) {
		self.rect.origin.x = value;
	}

	#[inline]
	pub fn set_y(&mut self, value: Pixel) {
		self.rect.origin.y = value;
	}

	#[inline]
	pub fn reset_right_sides(&mut self) {
		self.margin.right = PIXEL_ZERO;
		self.padding.right = PIXEL_ZERO;
	}
}

#[derive(Debug)]
pub struct TextFragment {
	pub dom_node: NodeRef,
	pub rect: Rect<Pixel, CSSPixel>,
	pub content: String,
}

impl Fragment for TextFragment {
	fn total_width(&self) -> Pixel {
		self.rect.width()
	}

	fn total_height(&self) -> Pixel {
		self.rect.height()
	}

	fn width(&self) -> Pixel {
		self.rect.width()
	}

	fn height(&self) -> Pixel {
		self.rect.height()
	}

	fn right_sides(&self) -> Pixel {
		PIXEL_ZERO
	}

	fn x(&self) -> Pixel {
		self.rect.origin.x
	}

	fn y(&self) -> Pixel {
		self.rect.origin.y
	}

	fn rect_x(&self) -> Pixel {
		self.x()
	}

	fn rect_y(&self) -> Pixel {
		self.y()
	}

	fn class(&self) -> FragmentClass {
		FragmentClass::TextFragment
	}

	fn as_text_fragment(&self) -> &TextFragment {
		self
	}

	fn build_display_list(&self, builder: &mut DisplayListBuilder, context: &mut BuilderContext) {
		let computed_values = GlobalScope::get_or_init_computed_values(self.dom_node.parent().unwrap().id());
		builder.push_text(
			Rect::new(Point2D::new(context.x + self.x(), context.y + self.y()), self.rect.size),
			&self.content,
			computed_values.get_color().clone(),
			computed_values.get_font_families(),
			computed_values.get_font_size(),
		)
	}
}

impl TextFragment {
	pub fn new(dom_node: NodeRef, content: String) -> Self {
		Self {
			dom_node,
			content,
			rect: Default::default(),
		}
	}

	#[inline]
	pub fn set_width(&mut self, value: Pixel) {
		self.rect.size.width = value;
	}

	#[inline]
	pub fn set_height(&mut self, value: Pixel) {
		self.rect.size.height = value;
	}

	#[inline]
	pub fn set_x(&mut self, value: Pixel) {
		self.rect.origin.x = value;
	}

	#[inline]
	pub fn set_y(&mut self, value: Pixel) {
		self.rect.origin.y = value;
	}
}

pub struct AnonymousFragment {
	pub rect: Rect<Pixel, CSSPixel>,
	pub bounds: Size2D<Pixel, CSSPixel>,
	pub children: Vec<Rc<RefCell<dyn Fragment>>>,
	pub lines: Rc<RefCell<Vec<Line>>>,
}

impl Fragment for AnonymousFragment {
	fn total_width(&self) -> Pixel {
		self.rect.width()
	}

	fn total_height(&self) -> Pixel {
		self.rect.height()
	}

	fn width(&self) -> Pixel {
		self.rect.width()
	}

	fn height(&self) -> Pixel {
		self.rect.height()
	}

	fn right_sides(&self) -> Pixel {
		PIXEL_ZERO
	}

	fn x(&self) -> Pixel {
		PIXEL_ZERO
	}

	fn y(&self) -> Pixel {
		self.rect.origin.y
	}

	fn rect_x(&self) -> Pixel {
		self.x()
	}

	fn rect_y(&self) -> Pixel {
		self.y()
	}

	fn class(&self) -> FragmentClass {
		FragmentClass::AnonymousFragment
	}

	fn as_anonymous_fragment(&self) -> &AnonymousFragment {
		self
	}

	fn build_display_list(&self, _builder: &mut DisplayListBuilder, _context: &mut BuilderContext) {}
}

impl AnonymousFragment {
	pub fn new(lines: Rc<RefCell<Vec<Line>>>) -> Self {
		Self {
			rect: Default::default(),
			bounds: Default::default(),
			children: Default::default(),
			lines,
		}
	}

	pub fn expandable_width(&self) -> Pixel {
		(self.bounds.width - self.rect.width()).max(PIXEL_ZERO)
	}

	#[inline]
	pub fn set_width(&mut self, value: Pixel) {
		self.rect.size.width = value;
	}

	#[inline]
	pub fn set_height(&mut self, value: Pixel) {
		self.rect.size.height = value;
	}

	#[inline]
	pub fn set_bounded_width(&mut self, value: Pixel) {
		self.bounds.width = value;
	}

	#[inline]
	pub fn set_bounded_height(&mut self, value: Pixel) {
		self.bounds.height = value;
	}

	pub fn set_y(&mut self, value: Pixel) {
		self.rect.origin.y = value;
	}
}
