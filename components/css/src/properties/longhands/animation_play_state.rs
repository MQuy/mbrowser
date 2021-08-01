#[derive(Clone)]
pub enum SingleAnimationPlayState {
    Running,
    Paused,
}

#[derive(Clone)]
pub struct AnimationPlayState {
    play_states: Vec<SingleAnimationPlayState>,
}
