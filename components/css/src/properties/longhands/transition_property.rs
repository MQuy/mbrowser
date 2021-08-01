use crate::values::CustomIdent;

#[derive(Clone)]
pub enum SingleTransitionProperty {
    All,
    Ident(CustomIdent),
}

#[derive(Clone)]
pub struct TransitionProperty {
    properties: Vec<SingleTransitionProperty>,
}
