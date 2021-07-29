use crate::values::length::LengthPercentageOrAuto;

#[derive(Clone)]
pub enum BackgroundSize {
    ExplicitSize {
        width: LengthPercentageOrAuto,
        height: LengthPercentageOrAuto,
    },
    Cover,
    Contain,
}
