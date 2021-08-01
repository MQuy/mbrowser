#[derive(Clone)]
#[repr(u8)]
pub enum OverflowWrap {
    Normal,
    BreakWord,
    Anywhere,
}
