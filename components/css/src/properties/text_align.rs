#[derive(Clone)]
#[repr(u8)]
pub enum TextAlignKeyword {
    Start,
    Left,
    Right,
    Center,
    Justify,
    End,
}

#[derive(Clone)]
pub enum TextAlign {
    Keyword(TextAlignKeyword),
}
