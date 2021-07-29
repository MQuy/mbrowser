#[derive(Clone)]
pub enum SingleAnimationDirection {
    Normal,
    Reverse,
    Alternate,
    AlternateReverse,
}

#[derive(Clone)]
pub struct AnimationDirection {
    directions: Vec<SingleAnimationDirection>,
}
