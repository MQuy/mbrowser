use super::layout::Size2D;
use super::length::{NonNegativeLength, NonNegativeLengthPercentage};

#[derive(Clone)]
pub enum BorderSideWidth {
    Thin,
    Medium,
    Thick,
    Length(NonNegativeLength),
}

#[derive(Clone)]
#[repr(C)]
pub struct GenericBorderCornerRadius<L>(pub Size2D<L>);

pub type BorderCornerRadius = GenericBorderCornerRadius<NonNegativeLengthPercentage>;
