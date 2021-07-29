use crate::values::number::Number;

#[derive(Clone)]
pub enum AbsoluteFontWeight {
    Weight(Number),
    Normal,
    Bold,
}

#[derive(Clone)]
pub enum FontWeight {
    Absolute(AbsoluteFontWeight),
    Bolder,
    Lighter,
}
