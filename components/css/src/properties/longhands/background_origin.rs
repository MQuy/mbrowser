#[derive(Clone)]
pub enum Box {
    PaddingBox,
    BorderBox,
    ContentBox,
}

#[derive(Clone)]
pub struct BackgroundOrigin {
    boxes: Vec<Box>,
}
