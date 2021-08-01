use crate::values::length::NonNegativeLengthPercentage;
use crate::values::number::NonNegativeNumber;

#[derive(Clone)]
#[repr(C, u8)]
pub enum LineHeight {
    Normal,
    Number(NonNegativeNumber),
    Length(NonNegativeLengthPercentage),
}
