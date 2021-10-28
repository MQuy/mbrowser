use std::any::Any;
use std::cell::RefMut;
use std::rc::Rc;

use common::not_supported;
use css::values::computed::length::{LengthPercentage, LengthPercentageOrAuto, Size};
use css::values::{Pixel, PIXEL_ZERO};
use dom::global_scope::{GlobalScope, NodeRef};

use super::boxes::{BaseBox, Box, BoxClass, SimpleBoxIterator};
use super::dimension::BoxDimension;
use super::formatting_context::{FormattingContext, FormattingContextType};

/// https://www.w3.org/TR/CSS22/visuren.html#block-boxes
pub struct BlockLevelBox {
	dom_node: NodeRef,
	base: BaseBox,
}

impl BlockLevelBox {
	pub fn new(dom_node: NodeRef, formatting_context: Rc<FormattingContext>) -> Self {
		BlockLevelBox {
			base: BaseBox::new(formatting_context),
			dom_node,
		}
	}

	pub fn dom_node(&self) -> NodeRef {
		self.dom_node.clone()
	}
}

impl Box for BlockLevelBox {
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

	fn size(&self) -> RefMut<'_, BoxDimension> {
		self.base.size()
	}

	fn get_total_width(&self) -> Pixel {
		let dimensions = self.base.size.borrow();
		dimensions.margin.margin_left
			+ dimensions.padding.padding_left
			+ dimensions.width
			+ dimensions.padding.padding_right
			+ dimensions.margin.margin_right
	}

	fn get_total_height(&self) -> Pixel {
		let dimensions = self.base.size.borrow();
		dimensions.margin.margin_top
			+ dimensions.padding.padding_top
			+ dimensions.height
			+ dimensions.padding.padding_bottom
			+ dimensions.margin.margin_bottom
	}

	fn is_block_container(&self) -> bool {
		true
	}

	fn compute_horizontal_used_value(&self) {
		let computed_values = GlobalScope::get_or_init_computed_values(self.dom_node.id());
		let containing_block = self
			.containing_block()
			.expect("has to have a containing block");
		let containing_width = containing_block.size().width;
		let containing_height = containing_block.size().height;
		let padding_left = computed_values
			.get_padding_left()
			.to_used_value(containing_width);
		let padding_right = computed_values
			.get_padding_right()
			.to_used_value(containing_width);
		let mut margin_left = PIXEL_ZERO;
		let mut margin_right = PIXEL_ZERO;
		let width = match computed_values.get_width() {
			Size::Auto => {
				margin_left = computed_values
					.get_margin_left()
					.to_used_value(containing_width, PIXEL_ZERO);
				margin_right = computed_values
					.get_margin_right()
					.to_used_value(containing_width, PIXEL_ZERO);
				PIXEL_ZERO.max(
					containing_width - margin_left - padding_left - padding_right - margin_right,
				)
			},
			Size::LengthPercentage(length_percentage) => {
				let width = length_percentage.to_used_value(containing_width);
				let margin = containing_width - width - padding_left - padding_right;
				if margin <= PIXEL_ZERO {
					if *computed_values.get_margin_left() == LengthPercentageOrAuto::Auto {
						margin_left = PIXEL_ZERO;
					}
					if *computed_values.get_margin_right() == LengthPercentageOrAuto::Auto {
						margin_right = PIXEL_ZERO;
					}
				} else {
					if *computed_values.get_margin_left() == LengthPercentageOrAuto::Auto
						&& *computed_values.get_margin_right() == LengthPercentageOrAuto::Auto
					{
						margin_left = margin / 2.0;
						margin_right = margin / 2.0;
					} else if *computed_values.get_margin_left() == LengthPercentageOrAuto::Auto {
						margin_right = computed_values
							.get_margin_right()
							.to_used_value(containing_width, PIXEL_ZERO);
						margin_left = margin - margin_right;
					} else if *computed_values.get_margin_right() == LengthPercentageOrAuto::Auto {
						margin_left = computed_values
							.get_margin_left()
							.to_used_value(containing_width, PIXEL_ZERO);
						margin_right = margin - margin_left;
					}
				};
				width
			},
			css::values::generics::length::GenericSize::ExtremumLength(_) => {
				not_supported!()
			},
		};
		let height = match computed_values.get_height() {
			Size::LengthPercentage(length_percentage) => match &length_percentage.0 {
				LengthPercentage::AbsoluteLength(value) => Pixel::new(*value),
				LengthPercentage::Percentage(percentage) if containing_height != PIXEL_ZERO => {
					containing_height * percentage.to_value(&(0.0..1.0))
				},
				_ => PIXEL_ZERO,
			},
			_ => PIXEL_ZERO,
		};
		let mut dimentions = self.size();
		dimentions.set_padding_left(padding_left);
		dimentions.set_padding_right(padding_right);
		dimentions.set_margin_left(margin_left);
		dimentions.set_margin_right(margin_right);
		dimentions.set_width(width);
		dimentions.set_height(height);
	}

	fn compute_vertical_used_value(&self) {
		let computed_values = GlobalScope::get_or_init_computed_values(self.dom_node.id());
		let containing_block = self
			.containing_block()
			.expect("has to have a containing block");
		let containing_width = containing_block.size().width;
		let containing_height = containing_block.size().height;
		let containing_constructing_height = containing_block.size().constructing_height;
		let padding_top = computed_values
			.get_padding_top()
			.to_used_value(containing_width);
		let padding_bottom = computed_values
			.get_padding_bottom()
			.to_used_value(containing_width);
		let margin_top = computed_values
			.get_margin_top()
			.to_used_value(containing_width, PIXEL_ZERO);
		let margin_bottom = computed_values
			.get_margin_bottom()
			.to_used_value(containing_width, PIXEL_ZERO);
		let height = match computed_values.get_height() {
			Size::Auto => BoxClass::get_total_children_height(self),
			Size::LengthPercentage(length_percentage) => match &length_percentage.0 {
				LengthPercentage::AbsoluteLength(value) => Pixel::new(*value),
				LengthPercentage::Percentage(percentage) if containing_height != PIXEL_ZERO => {
					containing_height * percentage.to_value(&(0.0..1.0))
				},
				_ => BoxClass::get_total_children_height(self),
			},
			Size::ExtremumLength(_) => not_supported!(),
		};
		let mut dimentions = self.size();
		dimentions.set_padding_top(padding_top);
		dimentions.set_padding_bottom(padding_bottom);
		dimentions.set_margin_top(margin_top);
		dimentions.set_margin_botom(margin_bottom);
		dimentions.set_height(height);
		dimentions.set_y(containing_constructing_height);
		containing_block.size().set_constructing_height(
			containing_constructing_height
				+ margin_top + padding_top
				+ height + padding_bottom
				+ margin_bottom,
		);
	}

	fn class(&self) -> BoxClass {
		BoxClass::Block
	}

	fn as_any(&self) -> &dyn Any {
		self
	}
}
