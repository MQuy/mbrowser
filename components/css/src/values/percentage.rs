use super::number::NonNegativeNumber;
use super::CSSFloat;

#[derive(Clone, Debug)]
pub struct Percentage {
    value: CSSFloat,
}

/// A computed <ratio> value.
#[derive(Clone, PartialEq, Debug)]
pub struct Ratio(pub NonNegativeNumber, pub NonNegativeNumber);
