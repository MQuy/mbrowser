use crate::values::length::LengthPercentage;

#[derive(Clone)]
pub enum FontSize {
    Length(LengthPercentage),
    Smaller,
    Larger,
}
