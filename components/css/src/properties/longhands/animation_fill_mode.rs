#[derive(Clone)]
pub enum SingleAnimationFillMode {
    None,
    Forwards,
    Backwards,
    Both,
}

#[derive(Clone)]
pub struct AnimationFillMode {
    fill_modes: Vec<SingleAnimationFillMode>,
}
