use std::rc::Rc;

use css::values::specified::color::RGBA;

use super::display_item::{DisplayItem, LayoutRect, RectangleDisplayItem, TextDisplayItem};
use crate::flow::tree::{BoxTree, PreOrderBoxTreeIterator};

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
		let node_iter = PreOrderBoxTreeIterator::new(box_tree.root.clone());
		for node in node_iter {
			node.build_display_list(&mut builder);
		}
		builder
	}

	pub fn push_rect(&mut self, bounds: LayoutRect, color: RGBA) {
		self.items
			.push(DisplayItem::Rectangle(RectangleDisplayItem {
				bounds,
				color,
			}))
	}

	pub fn push_text(
		&mut self,
		bounds: LayoutRect,
		content: &str,
		color: RGBA,
		font_families: &Vec<String>,
	) {
		self.items.push(DisplayItem::Text(TextDisplayItem {
			bounds,
			content: content.to_string(),
			color,
			font_families: font_families.clone(),
		}))
	}
}
