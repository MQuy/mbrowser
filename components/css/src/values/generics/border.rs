use crate::values::length::Pair;

#[derive(Clone)]
#[repr(C)]
pub struct GenericBorderCornerRadius<L>(pub Pair<L>);
