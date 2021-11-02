use std::any::Any;
use std::cell::RefMut;
use std::rc::Rc;

use common::not_supported;
use css::values::computed::length::{LengthPercentage, Size};
use css::values::{Pixel, PIXEL_ZERO};
use dom::characterdata::CharacterData;
use dom::global_scope::{GlobalScope, NodeRef};
use dom::inheritance::Castable;
use euclid::{Point2D, Size2D};

use super::boxes::{BaseBox, Box, BoxClass, SimpleBoxIterator};
use super::dimension::BoxDimension;
use super::formatting_context::{FormattingContext, FormattingContextType};
use crate::display_list::builder::DisplayListBuilder;
use crate::display_list::display_item::LayoutRect;
use crate::text::TextUI;

/// https://www.w3.org/TR/CSS22/visuren.html#inline-boxes
pub struct InlineLevelBox {
	dom_node: NodeRef,
	base: BaseBox,
}

impl InlineLevelBox {
	pub fn new(dom_node: NodeRef, formatting_context: Rc<FormattingContext>) -> Self {
		InlineLevelBox {
			base: BaseBox::new(formatting_context),
			dom_node,
		}
	}

	pub fn dom_node(&self) -> NodeRef {
		self.dom_node.clone()
	}
}

impl Box for InlineLevelBox {
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
		self.base.size.borrow().height
	}

	fn is_block_container(&self) -> bool {
		self.formatting_context_type() == FormattingContextType::BlockFormattingContext
	}

	fn compute_horizontal_used_value(&self) {
		let computed_values = GlobalScope::get_or_init_computed_values(self.dom_node.id());
		let containing_block = self
			.containing_block()
			.expect("has to have a containing block");
		let containing_width = containing_block.size().width;
		let padding_left = computed_values
			.get_padding_left()
			.to_used_value(containing_width);
		let padding_right = computed_values
			.get_padding_right()
			.to_used_value(containing_width);
		let margin_left = computed_values
			.get_margin_left()
			.to_used_value(containing_width, PIXEL_ZERO);
		let margin_right = computed_values
			.get_margin_right()
			.to_used_value(containing_width, PIXEL_ZERO);
		let width = match self.formatting_context_type() {
			FormattingContextType::BlockFormattingContext => match computed_values.get_width() {
				Size::Auto => BoxClass::get_total_children_intrinsic_width(self).min(
					containing_width - margin_left - padding_left - padding_right - margin_right,
				),
				Size::LengthPercentage(value) => match value.0.clone() {
					LengthPercentage::AbsoluteLength(length) => Pixel::new(length),
					LengthPercentage::Percentage(percentage) => {
						containing_width * percentage.to_value(&(0.0..1.0))
					},
				},
				_ => not_supported!(),
			},
			FormattingContextType::InlineFormattingContext => {
				self.size().intrinsic_width.min(containing_width)
			},
		};
		let mut dimentions = self.size();
		dimentions.set_padding_left(padding_left);
		dimentions.set_padding_right(padding_right);
		dimentions.set_margin_left(margin_left);
		dimentions.set_margin_right(margin_right);
		dimentions.set_width(width);

		let parent = self.parent().expect("has to have a parent");
		let parent_constructing_width = parent.size().constructing_width;
		dimentions.set_x(parent_constructing_width);
		parent.size().set_constructing_width(
			parent_constructing_width
				+ margin_left + padding_left
				+ width + padding_right
				+ margin_right,
		);
	}

	fn compute_vertical_used_value(&self) {
		let containing_block = self
			.containing_block()
			.expect("has to have a containing block");
		let containing_width = containing_block.size().width;
		let height = if self.dom_node.node_type_id().is_character_data_text() {
			let content = self.dom_node.0.downcast::<CharacterData>().data();
			let computed_values = GlobalScope::get_or_init_computed_values(
				self.dom_node
					.parent_node()
					.expect("dom has to have a parent")
					.id(),
			);
			let family_names = computed_values.get_font_families();
			Pixel::new(
				TextUI::new()
					.measure_size_in_bounded(
						content.as_str(),
						family_names,
						14.0,
						(self.size().width.0, f32::MAX),
					)
					.1,
			)
		} else {
			let computed_values = GlobalScope::get_or_init_computed_values(self.dom_node.id());
			match self.formatting_context_type() {
				FormattingContextType::BlockFormattingContext => match computed_values.get_height()
				{
					Size::Auto => BoxClass::get_total_children_height(self),
					Size::LengthPercentage(length_percentage) => match &length_percentage.0 {
						LengthPercentage::AbsoluteLength(length) => Pixel::new(*length),
						LengthPercentage::Percentage(percentage) => {
							containing_width * percentage.to_value(&(0.0..1.0))
						},
					},
					Size::ExtremumLength(_) => not_supported!(),
				},
				FormattingContextType::InlineFormattingContext => {
					BoxClass::get_total_children_height(self)
				},
			}
		};
		let mut dimentions = self.size();
		dimentions.set_height(height);
	}

	fn class(&self) -> BoxClass {
		BoxClass::Inline
	}

	fn as_any(&self) -> &dyn Any {
		self
	}

	fn build_display_list(&self, builder: &mut DisplayListBuilder) {
		let containing_block = self.containing_block().unwrap();
		let (px, py) = BoxClass::get_absolute_axis(containing_block.clone());
		let containing_dimension = containing_block.size();
		let dimension = self.base.size();
		if self.dom_node().node_type_id().is_character_data_text() {
			let parent_node = self.dom_node.0.parent_node().unwrap();
			let parent_computed_values = GlobalScope::get_or_init_computed_values(parent_node.id());
			let content = self.dom_node.0.downcast::<CharacterData>().data();
			builder.push_text(
				LayoutRect::new(
					Point2D::new(
						px + containing_dimension.margin.margin_left
							+ containing_dimension.padding.padding_left
							+ dimension.x + dimension.margin.margin_left
							+ dimension.padding.padding_left,
						py + containing_dimension.margin.margin_top
							+ containing_dimension.padding.padding_top
							+ dimension.y + dimension.margin.margin_top
							+ dimension.padding.padding_top,
					),
					Size2D::new(dimension.width, dimension.height),
				),
				content.as_str(),
				parent_computed_values.get_color().clone(),
			)
		} else {
			let computed_values = GlobalScope::get_or_init_computed_values(self.dom_node().id());
			builder.push_rect(
				LayoutRect::new(
					Point2D::new(
						px + containing_dimension.margin.margin_left
							+ containing_dimension.padding.padding_left
							+ dimension.x + dimension.margin.margin_left
							+ dimension.padding.padding_left,
						py + containing_dimension.margin.margin_top
							+ containing_dimension.padding.padding_top
							+ dimension.y + dimension.margin.margin_top
							+ dimension.padding.padding_top,
					),
					Size2D::new(dimension.width, dimension.height),
				),
				computed_values.get_background_color().clone(),
			)
		}
	}
}
