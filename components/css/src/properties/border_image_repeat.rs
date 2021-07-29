#[derive(Clone)]
pub enum BorderImageRepeatKeyword {
    Stretch,
    Repeat,
    Round,
    Space,
}

#[derive(Clone)]
pub struct BorderImageRepeat(pub BorderImageRepeatKeyword, pub BorderImageRepeatKeyword);
