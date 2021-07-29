#[derive(Clone)]
pub enum BackgroundRepeatKeyword {
    Repeat,
    Space,
    Round,
    NoRepeat,
}

#[derive(Clone)]
pub struct BackgroundRepeat(pub BackgroundRepeatKeyword, pub BackgroundRepeatKeyword);
