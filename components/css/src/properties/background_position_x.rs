use crate::values::length::LengthPercentage;

#[derive(Clone)]
pub enum HorizontalPositionKeyword {
    Left,
    Right,
    XStart,
    XEnd,
}

#[derive(Clone)]
pub struct HorizontalPosition {
    keyword: Option<HorizontalPositionKeyword>,
    length: Option<LengthPercentage>,
}

#[derive(Clone)]
pub enum HorizontalPositionComponent {
    Center,
    PositionX,
}

#[derive(Clone)]
pub struct BackgroundPositionX {
    positions: Vec<HorizontalPositionComponent>,
}
