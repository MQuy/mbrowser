use crate::values::time::Time;

#[derive(Clone)]
pub struct TransitionDuration {
    delays: Vec<Time>,
}
