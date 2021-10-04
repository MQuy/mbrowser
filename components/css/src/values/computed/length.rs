use crate::values::generics::length::{GenericLengthPercentageOrAuto, GenericSize};
use crate::values::generics::number::NonNegative;
use crate::values::specified::percentage::Percentage;
use crate::values::CSSFloat;

/// https://www.w3.org/TR/css-values-4/#typedef-length-percentage
/// <length-percentage> = [ px | <percentage> ]
#[derive(Clone, Debug)]
pub enum LengthPercentage {
	AbsoluteLength(CSSFloat),
	Percentage(Percentage),
}

/// value = <length> | <percentage> | auto
pub type LengthPercentageOrAuto = GenericLengthPercentageOrAuto<LengthPercentage>;

/// value = <length [0, ∞]> | <percentage>
pub type NonNegativeLengthPercentage = NonNegative<LengthPercentage>;

/// value = <length [0, ∞]> | <percentage>
pub type NonNegativeLengthPercentageOrAuto = NonNegative<LengthPercentageOrAuto>;

pub type Size = GenericSize<NonNegativeLengthPercentage>;
