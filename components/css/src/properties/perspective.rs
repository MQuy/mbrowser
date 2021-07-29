use crate::values::length::NonNegativeLength;

#[derive(Clone)]
#[repr(C, u8)]
pub enum Perspective {
    Length(NonNegativeLength),
    None,
}
