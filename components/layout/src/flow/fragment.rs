use std::rc::Rc;

use css::computed_values::ComputedValues;
use css::values::{CSSPixel, Pixel, PIXEL_ZERO};
use dom::global_scope::NodeRef;
use euclid::{Rect, Size2D};

use super::boxes::Box;
use super::formatting_context::FormattingContextType;

pub struct Line {
	pub fragments: Vec<Rc<Fragment>>,
	pub bounds: Rect<Pixel, CSSPixel>,
}

impl Line {
	pub fn new() -> Self {
		todo!()
	}
}

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
	pub preferred_minimum_width: Pixel,
}

impl Default for IntrinsicSize {
	fn default() -> Self {
		Self {
			preferred_width: PIXEL_ZERO,
			preferred_minimum_width: PIXEL_ZERO,
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
	pub fn horizontal_sides(&self) -> Pixel {
		self.margin.left + self.padding.left + self.padding.right + self.margin.right
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
						child_layout_info.intrinsic_size.preferred_minimum_width
							+ child_layout_info.horizontal_sides(),
					);
					preferred_width = preferred_width
						.max(child_layout_info.intrinsic_size.preferred_width)
						+ child_layout_info.horizontal_sides();
				}
			},
			FormattingContextType::InlineFormattingContext => {
				for child in node.children() {
					let child_layout_info = child.layout_info();
					preferred_minimum_width = preferred_minimum_width.max(
						child_layout_info.intrinsic_size.preferred_minimum_width
							+ child_layout_info.horizontal_sides(),
					);
					preferred_width += child_layout_info.intrinsic_size.preferred_width
						+ child_layout_info.horizontal_sides();
				}
			},
		};
		self.intrinsic_size.preferred_minimum_width = preferred_minimum_width;
		self.intrinsic_size.preferred_width = preferred_width;
	}
}

pub enum Fragment {
	BoxFragment(BoxFragment),
	TextFragment(TextFragment),
	AnonymousFragment(AnonymousFragment),
}

impl Fragment {
	pub fn rect(&self) -> Rect<Pixel, CSSPixel> {
		match self {
			Fragment::BoxFragment(value) => value.rect,
			Fragment::TextFragment(value) => value.rect,
			Fragment::AnonymousFragment(value) => value.rect,
		}
	}

	/// contect rect + margin + padding
	pub fn total_width(&self) -> Pixel {
		match self {
			Fragment::BoxFragment(value) => {
				value.margin.left
					+ value.padding.left + value.rect.width()
					+ value.padding.right
					+ value.margin.right
			},

			Fragment::TextFragment(value) => value.rect.width(),
			Fragment::AnonymousFragment(value) => value.rect.width(),
		}
	}

	pub fn total_height(&self) -> Pixel {
		match self {
			Fragment::BoxFragment(value) => {
				value.margin.top
					+ value.padding.top + value.rect.height()
					+ value.padding.bottom
					+ value.margin.bottom
			},

			Fragment::TextFragment(value) => value.rect.height(),
			Fragment::AnonymousFragment(value) => value.rect.height(),
		}
	}
}

pub struct BoxFragment {
	pub dom_node: NodeRef,
	pub padding: Sides,
	pub margin: Sides,
	pub rect: Rect<Pixel, CSSPixel>,
	pub bounds: Size2D<Pixel, CSSPixel>,
}

impl BoxFragment {
	pub fn new(dom_node: NodeRef) -> Self {
		BoxFragment {
			dom_node,
			rect: Default::default(),
			padding: Default::default(),
			margin: Default::default(),
			bounds: Default::default(),
		}
	}

	pub fn total_width(&self) -> Pixel {
		self.margin.left
			+ self.padding.left
			+ self.rect.width()
			+ self.padding.right
			+ self.margin.right
	}

	pub fn total_height(&self) -> Pixel {
		self.margin.top
			+ self.padding.top
			+ self.rect.height()
			+ self.padding.bottom
			+ self.margin.bottom
	}

	pub fn expandable_width(&self) -> Pixel {
		(self.bounds.width - self.rect.width()).max(PIXEL_ZERO)
	}
}

pub struct TextFragment {
	pub dom_node: NodeRef,
	pub rect: Rect<Pixel, CSSPixel>,
	pub content: String,
}

impl TextFragment {
	pub fn new() -> Self {
		todo!()
	}
}

pub struct AnonymousFragment {
	pub rect: Rect<Pixel, CSSPixel>,
	pub bounds: Size2D<Pixel, CSSPixel>,
}

impl AnonymousFragment {
	pub fn new() -> Self {
		todo!()
	}
}
