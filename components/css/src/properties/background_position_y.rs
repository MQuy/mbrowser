use crate::values::length::LengthPercentage;

#[derive(Clone)]
pub enum VerticalPositionKeyword {
    Left,
    Right,
    YStart,
    YEnd,
}

#[derive(Clone)]
pub struct VerticalPosition {
    keyword: Option<VerticalPositionKeyword>,
    length: Option<LengthPercentage>,
}

#[derive(Clone)]
pub enum VerticalPositionComponent {
    Center,
    PositionX,
}

#[derive(Clone)]
pub struct BackgroundPositionY {
    positions: Vec<VerticalPositionComponent>,
}
