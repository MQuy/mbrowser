use crate::values::length::LengthPercentage;

#[derive(Clone)]
#[repr(u8)]
pub enum VerticalAlignKeyword {
    Baseline,
    Sub,
    Super,
    Top,
    TextTop,
    Middle,
    Bottom,
    TextBottom,
}

#[derive(Clone)]
pub enum VerticalAlign {
    VerticalAlignKeyword,
    LengthPercentage(LengthPercentage),
}
