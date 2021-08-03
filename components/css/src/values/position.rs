use super::percentage::Ratio;

#[derive(Clone)]
#[repr(C, u8)]
pub enum PreferredRatio {
    None,
    Ratio(Ratio),
}
