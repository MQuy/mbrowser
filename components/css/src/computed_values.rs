use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::properties::declaration::PropertyDeclaration;
use crate::properties::longhand_id::LonghandId;
use crate::properties::longhands::display::Display;
use crate::stylesheets::origin::Origin;
use crate::values::length::LengthPercentageOrAuto;

#[derive(Debug)]
pub struct Margin {
	pub margin_top: LengthPercentageOrAuto,
	pub margin_right: LengthPercentageOrAuto,
	pub margin_bottom: LengthPercentageOrAuto,
	pub margin_left: LengthPercentageOrAuto,
}

#[derive(Debug)]
pub struct Padding {
	pub margin_top: LengthPercentageOrAuto,
	pub margin_right: LengthPercentageOrAuto,
	pub margin_bottom: LengthPercentageOrAuto,
	pub margin_left: LengthPercentageOrAuto,
}

#[derive(Debug)]
pub struct Box {
	pub display: Display,
}

#[derive(Debug)]
pub struct ComputedValues {
	box_: Rc<Box>,
	margin: Rc<Margin>,
	padding: Rc<Padding>,
}

impl Default for ComputedValues {
	fn default() -> Self {
		todo!()
	}
}

impl ComputedValues {
	pub fn set_margin_top(&self) {}
}

pub struct ElementStyles {
	primary: ComputedValues,
}

pub struct StyleContext<'a, 'b, 'c> {
	pub parent_style: Option<&'c ComputedValues>,
	pub cascade_data: HashMap<LonghandId, PropertyCascade<'a>>,
	pub computed_values: &'b mut ComputedValues,
}

pub struct PropertyCascade<'a> {
	pub origin: Origin,
	pub specificity: u32,
	pub importance: bool,
	pub property: &'a PropertyDeclaration,
}
