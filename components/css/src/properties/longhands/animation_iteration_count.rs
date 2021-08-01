use crate::values::number::Number;

#[derive(Clone)]
pub enum SingleAnimationIterationCount {
    Number(Number),
    Infinite,
}

#[derive(Clone)]
pub struct AnimationIterationCount {
    iteration_count: Vec<SingleAnimationIterationCount>,
}
