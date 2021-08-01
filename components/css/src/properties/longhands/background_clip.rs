#[derive(Clone)]
pub enum Box {
    BorderBox,
    PaddingBox,
    ContentBox,
}

#[derive(Clone)]
pub struct BackgroundClip {
    boxes: Vec<Box>,
}
