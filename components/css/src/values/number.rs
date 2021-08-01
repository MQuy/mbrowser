use super::CSSFloat;

#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub struct Number {
    value: CSSFloat,
}

#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub struct NonNegative<T>(pub T);

#[derive(Clone, PartialEq, PartialOrd)]
pub struct GreaterThanOrEqualToOne<T>(pub T);

pub type PositiveInteger = GreaterThanOrEqualToOne<Integer>;

pub type Integer = i32;

pub type NonNegativeNumber = NonNegative<Number>;
