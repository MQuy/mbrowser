use super::CSSFloat;

#[derive(Clone)]
pub enum TimeUnit {
    Second,
    Millisecond,
}

#[derive(Clone)]
pub struct Time {
    amount: CSSFloat,
    unit: TimeUnit,
}
