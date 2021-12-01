use std::cell::{Ref, RefCell};
use std::rc::Rc;

use css::values::specified::color::RGBA;
use css::values::{CSSFloat, Pixel, PIXEL_ZERO};

use super::display_item::{DisplayItem, LayoutRect, RectangleDisplayItem, TextDisplayItem};
use crate::flow::fragment::{Fragment, FragmentClass, Line};
use crate::flow::tree::BoxTree;

#[derive(Debug)]
pub struct BuilderContext {
	pub x: Pixel,
	pub y: Pixel,
}

pub struct DisplayListBuilder {
	pub items: Vec<DisplayItem>,
}

impl DisplayListBuilder {
	pub fn new() -> Self {
		DisplayListBuilder {
			items: Vec::with_capacity(1),
		}
	}

	pub fn construct(box_tree: Rc<BoxTree>) -> Self {
		let mut builder = DisplayListBuilder::new();
		let mut context = BuilderContext {
			x: PIXEL_ZERO,
			y: PIXEL_ZERO,
		};
		builder.construct_fragment(box_tree.root.as_block_level_box().fragment(), &mut context);
		builder
	}

	fn construct_fragment(&mut self, fragment: Ref<dyn Fragment>, context: &mut BuilderContext) {
		fragment.build_display_list(self, context);
		let mut child_context = BuilderContext {
			x: context.x + fragment.rect_x(),
			y: context.y + fragment.rect_y(),
		};
		match fragment.class() {
			FragmentClass::BoxFragment => {
				let box_fragment = fragment.as_box_fragment();
				self.construct_children(&mut child_context, box_fragment.lines.borrow(), &box_fragment.children);
			},
			FragmentClass::TextFragment => {
				fragment.as_text_fragment().build_display_list(self, context);
			},
			FragmentClass::AnonymousFragment => {
				let anonymous_fragment = fragment.as_anonymous_fragment();
				self.construct_children(
					&mut child_context,
					anonymous_fragment.lines.borrow(),
					&anonymous_fragment.children,
				);
			},
		};
	}

	fn construct_children(
		&mut self,
		context: &mut BuilderContext,
		lines: Ref<Vec<Line>>,
		children: &Vec<Rc<RefCell<dyn Fragment>>>,
	) {
		if lines.len() > 0 {
			for line in lines.iter() {
				let mut line_context = BuilderContext {
					x: context.x,
					y: context.y + line.y(),
				};
				for fragment in line.fragments().iter() {
					self.construct_fragment(fragment.borrow(), &mut line_context);
				}
			}
		} else {
			for child_fragment in children.iter() {
				self.construct_fragment(child_fragment.borrow(), context);
			}
		}
	}

	pub fn push_rect(&mut self, bounds: LayoutRect, color: RGBA) {
		self.items
			.push(DisplayItem::Rectangle(RectangleDisplayItem { bounds, color }))
	}

	pub fn push_text(
		&mut self,
		bounds: LayoutRect,
		content: &str,
		color: RGBA,
		font_families: &Vec<String>,
		font_size: CSSFloat,
	) {
		self.items.push(DisplayItem::Text(TextDisplayItem {
			bounds,
			content: content.to_string(),
			color,
			font_size,
			font_families: font_families.clone(),
		}))
	}
}
