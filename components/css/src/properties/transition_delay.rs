use crate::values::time::Time;

#[derive(Clone)]
pub struct TransitionDelay {
    delays: Vec<Time>,
}
