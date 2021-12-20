use common::not_supported;

use crate::values::generics::length::{GenericLengthPercentageOrAuto, GenericMaxSize, GenericSize};
use crate::values::generics::number::NonNegative;
use crate::values::specified::percentage::Percentage;
use crate::values::{CSSFloat, Pixel};

/// https://www.w3.org/TR/css-values-4/#typedef-length-percentage
/// <length-percentage> = [ px | <percentage> ]
#[derive(Clone, Debug, PartialEq)]
pub enum LengthPercentage {
	AbsoluteLength(CSSFloat),
	Percentage(Percentage),
}

/// value = <length> | <percentage> | auto
pub type LengthPercentageOrAuto = GenericLengthPercentageOrAuto<LengthPercentage>;

impl LengthPercentageOrAuto {
	pub fn zero() -> Self {
		LengthPercentageOrAuto::LengthPercentage(LengthPercentage::AbsoluteLength(0.0))
	}

	#[inline]
	pub fn to_used_value(&self, base_value: Pixel, fallback_value: Pixel) -> Pixel {
		match self {
			LengthPercentageOrAuto::LengthPercentage(length_percentage) => match length_percentage {
				LengthPercentage::AbsoluteLength(value) => Pixel::new(*value),
				LengthPercentage::Percentage(value) => base_value * value.to_value(&(0.0..1.0)),
			},
			LengthPercentageOrAuto::Auto => fallback_value,
		}
	}

	#[inline]
	pub fn to_fixed_used_value(&self) -> Option<Pixel> {
		match self {
			GenericLengthPercentageOrAuto::LengthPercentage(length_percentage) => match length_percentage {
				LengthPercentage::AbsoluteLength(length) => Some(Pixel::new(*length)),
				_ => None,
			},
			_ => None,
		}
	}
}

pub type NonNegativeLength = NonNegative<CSSFloat>;

/// value = <length [0, ∞]> | <percentage>
pub type NonNegativeLengthPercentage = NonNegative<LengthPercentage>;

impl NonNegativeLengthPercentage {
	pub fn zero() -> Self {
		NonNegative(LengthPercentage::AbsoluteLength(0.0))
	}

	#[inline]
	pub fn to_used_value(&self, base_value: Pixel) -> Pixel {
		match &self.0 {
			LengthPercentage::AbsoluteLength(value) => Pixel::new(*value),
			LengthPercentage::Percentage(value) => base_value * value.to_value(&(0.0..1.0)),
		}
	}

	#[inline]
	pub fn to_fixed_used_value(&self) -> Option<Pixel> {
		match self.0 {
			LengthPercentage::AbsoluteLength(length) => Some(Pixel::new(length)),
			_ => None,
		}
	}
}

/// value = <length [0, ∞]> | <percentage>
pub type NonNegativeLengthPercentageOrAuto = NonNegative<LengthPercentageOrAuto>;

pub type Size = GenericSize<NonNegativeLengthPercentage>;

impl Size {
	#[inline]
	pub fn to_fixed_used_value(&self) -> Option<Pixel> {
		match self {
			Self::Auto => None,
			Self::LengthPercentage(length_percentage) => match length_percentage.0 {
				LengthPercentage::AbsoluteLength(length) => Some(Pixel::new(length)),
				_ => None,
			},
			Self::ExtremumLength(_) => not_supported!(),
		}
	}
}

pub type MaxSize = GenericMaxSize<NonNegativeLengthPercentage>;

impl MaxSize {
	#[inline]
	pub fn to_fixed_used_value(&self) -> Option<Pixel> {
		match self {
			Self::None => None,
			Self::LengthPercentage(length_percentage) => match length_percentage.0 {
				LengthPercentage::AbsoluteLength(length) => Some(Pixel::new(length)),
				_ => None,
			},
			Self::ExtremumLength(_) => not_supported!(),
		}
	}
}
