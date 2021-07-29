use super::CSSFloat;

#[derive(Clone)]
pub struct Number {
    value: CSSFloat,
}

#[derive(Clone)]
pub struct NonNegative<T>(pub T);

#[derive(Clone)]
pub struct GreaterThanOrEqualToOne<T>(pub T);

pub type PositiveInteger = GreaterThanOrEqualToOne<Integer>;

pub type Integer = i32;

pub type NonNegativeNumber = NonNegative<Number>;
