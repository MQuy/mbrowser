use crate::values::length::{Length, LengthPercentage};

#[derive(Clone)]
pub enum Translate {
    None,
    LengthPercentage(LengthPercentage, LengthPercentageWithLength),
}

#[derive(Clone)]
pub struct LengthPercentageWithLength {
    length_percentage: LengthPercentage,
    length: Option<Length>,
}

#[derive(Clone)]
pub struct LengthPercentageComponent {
    length: Vec<LengthPercentageWithLength>,
}
