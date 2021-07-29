use crate::values::percentage::Percentage;

#[derive(Clone)]
pub enum FontStretchKeyword {
    Normal,
    Condensed,
    UltraCondensed,
    ExtraCondensed,
    SemiCondensed,
    SemiExpanded,
    Expanded,
    ExtraExpanded,
    UltraExpanded,
}

#[derive(Clone)]
#[repr(u8)]
pub enum FontStretch {
    Stretch(Percentage),
    Keyword(FontStretchKeyword),
}
