#[derive(Clone)]
#[repr(u8)]
pub enum WordBreak {
    Normal,
    BreakAll,
    KeepAll,
}
